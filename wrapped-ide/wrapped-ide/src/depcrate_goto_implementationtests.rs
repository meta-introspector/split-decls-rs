// Generated macro for tests (module)
macro_rules! Depcrate_goto_implementationtests {
() => {
// Module: crate::goto_implementation
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use ide_db :: FileRange ; use itertools :: Itertools ; use crate :: { GotoImplementationConfig , fixture } ; const TEST_CONFIG : & GotoImplementationConfig = & GotoImplementationConfig { filter_adjacent_derive_implementations : false } ; # [track_caller] fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str) { check_with_config (TEST_CONFIG , ra_fixture) ; } # [track_caller] fn check_with_config (config : & GotoImplementationConfig , # [rust_analyzer :: rust_fixture] ra_fixture : & str ,) { let (analysis , position , expected) = fixture :: annotations (ra_fixture) ; let navs = analysis . goto_implementation (config , position) . unwrap () . unwrap () . info ; let cmp = | frange : & FileRange | (frange . file_id , frange . range . start ()) ; let actual = navs . into_iter () . map (| nav | FileRange { file_id : nav . file_id , range : nav . focus_or_full_range () }) . sorted_by_key (cmp) . collect :: < Vec < _ > > () ; let expected = expected . into_iter () . map (| (range , _) | range) . sorted_by_key (cmp) . collect :: < Vec < _ > > () ; assert_eq ! (expected , actual) ; } # [test] fn goto_implementation_works () { check (r#"
struct Foo$0;
impl Foo {}
   //^^^
"# ,) ; } # [test] fn goto_implementation_works_multiple_blocks () { check (r#"
struct Foo$0;
impl Foo {}
   //^^^
impl Foo {}
   //^^^
"# ,) ; } # [test] fn goto_implementation_works_multiple_mods () { check (r#"
struct Foo$0;
mod a {
    impl super::Foo {}
       //^^^^^^^^^^
}
mod b {
    impl super::Foo {}
       //^^^^^^^^^^
}
"# ,) ; } # [test] fn goto_implementation_works_multiple_files () { check (r#"
//- /lib.rs
struct Foo$0;
mod a;
mod b;
//- /a.rs
impl crate::Foo {}
   //^^^^^^^^^^
//- /b.rs
impl crate::Foo {}
   //^^^^^^^^^^
"# ,) ; } # [test] fn goto_implementation_for_trait () { check (r#"
trait T$0 {}
struct Foo;
impl T for Foo {}
         //^^^
"# ,) ; } # [test] fn goto_implementation_for_trait_multiple_files () { check (r#"
//- /lib.rs
trait T$0 {};
struct Foo;
mod a;
mod b;
//- /a.rs
impl crate::T for crate::Foo {}
                //^^^^^^^^^^
//- /b.rs
impl crate::T for crate::Foo {}
                //^^^^^^^^^^
            "# ,) ; } # [test] fn goto_implementation_all_impls () { check (r#"
//- /lib.rs
trait T {}
struct Foo$0;
impl Foo {}
   //^^^
impl T for Foo {}
         //^^^
impl T for &Foo {}
"# ,) ; } # [test] fn goto_implementation_to_builtin_derive () { check (r#"
//- minicore: copy, derive
  #[derive(Copy)]
         //^^^^
struct Foo$0;
"# ,) ; } # [test] fn goto_implementation_type_alias () { check (r#"
struct Foo;

type Bar$0 = Foo;

impl Foo {}
   //^^^
impl Bar {}
   //^^^
"# ,) ; } # [test] fn goto_implementation_adt_generic () { check (r#"
struct Foo$0<T>;

impl<T> Foo<T> {}
      //^^^^^^
impl Foo<str> {}
   //^^^^^^^^
"# ,) ; } # [test] fn goto_implementation_builtin () { check (r#"
//- /lib.rs crate:main deps:core
fn foo(_: bool$0) {{}}
//- /libcore.rs crate:core
#![rustc_coherence_is_core]
#[lang = "bool"]
impl bool {}
   //^^^^
"# ,) ; } # [test] fn goto_implementation_trait_functions () { check (r#"
trait Tr {
    fn f$0();
}

struct S;

impl Tr for S {
    fn f() {
     //^
        println!("Hello, world!");
    }
}
"# ,) ; } # [test] fn goto_implementation_trait_assoc_const () { check (r#"
trait Tr {
    const C$0: usize;
}

struct S;

impl Tr for S {
    const C: usize = 4;
        //^
}
"# ,) ; } # [test] fn goto_adt_implementation_inside_block () { check (r#"
//- minicore: copy, derive
trait Bar {}

fn test() {
    #[derive(Copy)]
  //^^^^^^^^^^^^^^^
    struct Foo$0;

    impl Foo {}
       //^^^

    trait Baz {}

    impl Bar for Foo {}
               //^^^

    impl Baz for Foo {}
               //^^^
}
"# ,) ; } # [test] fn goto_trait_implementation_inside_block () { check (r#"
struct Bar;

fn test() {
    trait Foo$0 {}

    struct Baz;

    impl Foo for Bar {}
               //^^^

    impl Foo for Baz {}
               //^^^
}
"# ,) ; check (r#"
struct Bar;

fn test() {
    trait Foo {
        fn foo$0() {}
    }

    struct Baz;

    impl Foo for Bar {
        fn foo() {}
         //^^^
    }

    impl Foo for Baz {
        fn foo() {}
         //^^^
    }
}
"# ,) ; } # [test] fn filter_adjacent_derives () { check_with_config (& GotoImplementationConfig { filter_adjacent_derive_implementations : true } , r#"
//- minicore: clone, copy, derive

#[derive(Clone, Copy)]
struct Foo$0;

trait Bar {}

impl Bar for Foo {}
          // ^^^
            "# ,) ; } }
};
}
