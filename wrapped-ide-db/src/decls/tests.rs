macro_rules! deps {
    () => {
        FilePosition!();
        RootDatabase!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use expect_test :: { Expect , expect } ; use hir :: FilePosition ; use hir :: Semantics ; use span :: Edition ; use syntax :: ast :: { self , AstNode } ; use test_fixture :: ChangeFixture ; use crate :: RootDatabase ; # [doc = " Creates analysis from a multi-file fixture, returns positions marked with $0."] pub (crate) fn position (# [rust_analyzer :: rust_fixture] ra_fixture : & str ,) -> (RootDatabase , FilePosition) { let mut database = RootDatabase :: default () ; let change_fixture = ChangeFixture :: parse (& database , ra_fixture) ; database . apply_change (change_fixture . change) ; let (file_id , range_or_offset) = change_fixture . file_position . expect ("expected a marker ($0)") ; let offset = range_or_offset . expect_offset () ; (database , FilePosition { file_id , offset }) } fn check_trait (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let (db , position) = position (ra_fixture) ; let sema = Semantics :: new (& db) ; let file = sema . parse (position . file_id) ; let impl_block : ast :: Impl = sema . find_node_at_offset_with_descend (file . syntax () , position . offset) . unwrap () ; let trait_ = crate :: traits :: resolve_target_trait (& sema , & impl_block) ; let actual = match trait_ { Some (trait_) => trait_ . name (& db) . display (& db , Edition :: CURRENT) . to_string () , None => String :: new () , } ; expect . assert_eq (& actual) ; } fn check_missing_assoc (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let (db , position) = position (ra_fixture) ; let sema = Semantics :: new (& db) ; let file = sema . parse (position . file_id) ; let impl_block : ast :: Impl = sema . find_node_at_offset_with_descend (file . syntax () , position . offset) . unwrap () ; let items = crate :: traits :: get_missing_assoc_items (& sema , & impl_block) ; let actual = items . into_iter () . map (| item | item . name (& db) . unwrap () . display (& db , Edition :: CURRENT) . to_string ()) . collect :: < Vec < _ > > () . join ("\n") ; expect . assert_eq (& actual) ; } # [test] fn resolve_trait () { check_trait (r#"
pub trait Foo {
    fn bar();
}
impl Foo for u8 {
    $0
}
            "# , expect ! [["Foo"]] ,) ; check_trait (r#"
pub trait Foo {
    fn bar();
}
impl Foo for u8 {
    fn bar() {
        fn baz() {
            $0
        }
        baz();
    }
}
            "# , expect ! [["Foo"]] ,) ; check_trait (r#"
pub trait Foo {
    fn bar();
}
pub struct Bar;
impl Bar {
    $0
}
            "# , expect ! [[""]] ,) ; } # [test] fn missing_assoc_items () { check_missing_assoc (r#"
pub trait Foo {
    const FOO: u8;
    fn bar();
}
impl Foo for u8 {
    $0
}"# , expect ! [[r#"
                FOO
                bar"#]] ,) ; check_missing_assoc (r#"
pub trait Foo {
    const FOO: u8;
    fn bar();
}
impl Foo for u8 {
    const FOO: u8 = 10;
    $0
}"# , expect ! [[r#"
                bar"#]] ,) ; check_missing_assoc (r#"
pub trait Foo {
    const FOO: u8;
    fn bar();
}
impl Foo for u8 {
    const FOO: u8 = 10;
    fn bar() {$0}
}"# , expect ! [[r#""#]] ,) ; check_missing_assoc (r#"
pub struct Foo;
impl Foo {
    fn bar() {$0}
}"# , expect ! [[r#""#]] ,) ; check_missing_assoc (r#"
trait Tr {
    fn required();
}
macro_rules! m {
    () => { fn required() {} };
}
impl Tr for () {
    m!();
    $0
}

            "# , expect ! [[r#""#]] ,) ; } }
    };
}

tests!()