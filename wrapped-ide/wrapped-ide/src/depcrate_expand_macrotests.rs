// Generated macro for tests (module)
macro_rules! Depcrate_expand_macrotests {
() => {
// Module: crate::expand_macro
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use expect_test :: { Expect , expect } ; use crate :: fixture ; # [track_caller] fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let (analysis , pos) = fixture :: position (ra_fixture) ; let expansion = analysis . expand_macro (pos) . unwrap () . unwrap () ; let actual = format ! ("{}\n{}" , expansion . name , expansion . expansion) ; expect . assert_eq (& actual) ; } # [test] fn expand_allowed_builtin_macro () { check (r#"
//- minicore: concat
$0concat!("test", 10, 'b', true);"# , expect ! [[r#"
                concat!
                "test10btrue""#]] ,) ; } # [test] fn do_not_expand_disallowed_macro () { let (analysis , pos) = fixture :: position (r#"
//- minicore: asm
$0asm!("0x300, x0");"# ,) ; let expansion = analysis . expand_macro (pos) . unwrap () ; assert ! (expansion . is_none ()) ; } # [test] fn macro_expand_as_keyword () { check (r#"
macro_rules! bar {
    ($i:tt) => { $i as _ }
}
fn main() {
    let x: u64 = ba$0r!(5i64);
}
"# , expect ! [[r#"
                bar!
                5i64 as _"#]] ,) ; } # [test] fn macro_expand_underscore () { check (r#"
macro_rules! bar {
    ($i:tt) => { for _ in 0..$i {} }
}
fn main() {
    ba$0r!(42);
}
"# , expect ! [[r#"
                bar!
                for _ in 0..42{}"#]] ,) ; } # [test] fn macro_expand_recursive_expansion () { check (r#"
macro_rules! bar {
    () => { fn  b() {} }
}
macro_rules! foo {
    () => { bar!(); }
}
macro_rules! baz {
    () => { foo!(); }
}
f$0oo!();
"# , expect ! [[r#"
                foo!
                fn b(){}"#]] ,) ; } # [test] fn macro_expand_multiple_lines () { check (r#"
macro_rules! foo {
    () => {
        fn some_thing() -> u32 {
            let a = 0;
            a + 10
        }
    }
}
f$0oo!();
        "# , expect ! [[r#"
                foo!
                fn some_thing() -> u32 {
                    let a = 0;
                    a+10
                }"#]] ,) ; } # [test] fn macro_expand_match_ast () { check (r#"
macro_rules! match_ast {
    (match $node:ident { $($tt:tt)* }) => { match_ast!(match ($node) { $($tt)* }) };
    (match ($node:expr) {
        $( ast::$ast:ident($it:ident) => $res:block, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = ast::$ast::cast($node.clone()) $res else )*
        { $catch_all }
    }};
}

fn main() {
    mat$0ch_ast! {
        match container {
            ast::TraitDef(it) => {},
            ast::ImplDef(it) => {},
            _ => { continue },
        }
    }
}
"# , expect ! [[r#"
                match_ast!
                {
                    if let Some(it) = ast::TraitDef::cast(container.clone()){}
                    else if let Some(it) = ast::ImplDef::cast(container.clone()){}
                    else {
                        {
                            continue
                        }
                    }
                }"#]] ,) ; } # [test] fn macro_expand_match_ast_inside_let_statement () { check (r#"
macro_rules! match_ast {
    (match $node:ident { $($tt:tt)* }) => { match_ast!(match ($node) { $($tt)* }) };
    (match ($node:expr) {}) => {{}};
}

fn main() {
    let p = f(|it| {
        let res = mat$0ch_ast! { match c {}};
        Some(res)
    })?;
}
"# , expect ! [[r#"
                match_ast!
                {}"#]] ,) ; } # [test] fn macro_expand_inner_macro_rules () { check (r#"
macro_rules! foo {
    ($t:tt) => {{
        macro_rules! bar {
            () => {
                $t
            }
        }
        bar!()
    }};
}

fn main() {
    foo$0!(42);
}
            "# , expect ! [[r#"
                foo!
                {
                    macro_rules! bar {
                        () => {
                            42
                        }
                    }
                    42
                }"#]] ,) ; } # [test] fn macro_expand_inner_macro_fail_to_expand () { check (r#"
macro_rules! bar {
    (BAD) => {};
}
macro_rules! foo {
    () => {bar!()};
}

fn main() {
    let res = fo$0o!();
}
"# , expect ! [[r#"
                foo!
                Expansion had errors:
                expected ident: `BAD`

            "#]] ,) ; } # [test] fn macro_expand_with_dollar_crate () { check (r#"
#[macro_export]
macro_rules! bar {
    () => {0};
}
macro_rules! foo {
    () => {$crate::bar!()};
}

fn main() {
    let res = fo$0o!();
}
"# , expect ! [[r#"
                foo!
                0"#]] ,) ; } # [test] fn macro_expand_with_dyn_absolute_path () { check (r#"
macro_rules! foo {
    () => {fn f<T>(_: &dyn ::std::marker::Copy) {}};
}

fn main() {
    fo$0o!()
}
"# , expect ! [[r#"
                foo!
                fn f<T>(_: &dyn ::std::marker::Copy){}"#]] ,) ; } # [test] fn macro_expand_item_expansion_in_expression_call () { check (r#"
macro_rules! foo {
    () => {fn f<T>() {}};
}

fn main() {
    let res = fo$0o!();
}
"# , expect ! [[r#"
                foo!
                fn f<T>(){}"#]] ,) ; } # [test] fn macro_expand_derive () { check (r#"
//- proc_macros: identity
//- minicore: clone, derive

#[proc_macros::identity]
#[derive(C$0lone)]
struct Foo {}
"# , expect ! [[r#"
                Clone
                impl <>core::clone::Clone for Foo< >where {
                    fn clone(&self) -> Self {
                        match self {
                            Foo{}
                             => Foo{}
                            ,

                            }
                    }

                    }"#]] ,) ; } # [test] fn macro_expand_derive2 () { check (r#"
//- minicore: copy, clone, derive

#[derive(Cop$0y)]
#[derive(Clone)]
struct Foo {}
"# , expect ! [[r#"
                Copy
                impl <>core::marker::Copy for Foo< >where{}"#]] ,) ; } # [test] fn macro_expand_derive_multi () { check (r#"
//- minicore: copy, clone, derive

#[derive(Cop$0y, Clone)]
struct Foo {}
"# , expect ! [[r#"
                Copy
                impl <>core::marker::Copy for Foo< >where{}"#]] ,) ; check (r#"
//- minicore: copy, clone, derive

#[derive(Copy, Cl$0one)]
struct Foo {}
"# , expect ! [[r#"
                Clone
                impl <>core::clone::Clone for Foo< >where {
                    fn clone(&self) -> Self {
                        match self {
                            Foo{}
                             => Foo{}
                            ,

                            }
                    }

                    }"#]] ,) ; } # [test] fn dollar_crate () { check (r#"
//- /a.rs crate:a
pub struct Foo;
#[macro_export]
macro_rules! m {
    ( $i:ident ) => { $crate::Foo; $crate::Foo; $i::Foo; };
}
//- /b.rs crate:b deps:a
pub struct Foo;
#[macro_export]
macro_rules! m {
    () => { a::m!($crate); $crate::Foo; $crate::Foo; };
}
//- /c.rs crate:c deps:b,a
pub struct Foo;
#[macro_export]
macro_rules! m {
    () => { b::m!(); $crate::Foo; $crate::Foo; };
}
fn bar() {
    m$0!();
}
"# , expect ! [[r#"
m!
a::Foo;
a::Foo;
b::Foo;
;
b::Foo;
b::Foo;
;
crate::Foo;
crate::Foo;"#]] ,) ; } # [test] fn semi_glueing () { check (r#"
macro_rules! __log_value {
    ($key:ident :$capture:tt =) => {};
}

macro_rules! __log {
    ($key:tt $(:$capture:tt)? $(= $value:expr)?; $($arg:tt)+) => {
        __log_value!($key $(:$capture)* = $($value)*);
    };
}

__log!(written:%; "Test"$0);
    "# , expect ! [[r#"
                __log!
            "#]] ,) ; } # [test] fn assoc_call () { check (r#"
macro_rules! mac {
    () => { fn assoc() {} }
}
impl () {
    mac$0!();
}
    "# , expect ! [[r#"
                mac!
                fn assoc(){}"#]] ,) ; } # [test] fn eager () { check (r#"
//- minicore: concat
macro_rules! my_concat {
    ($head:expr, $($tail:tt)*) => { concat!($head, $($tail)*) };
}


fn test() {
    _ = my_concat!(
        conc$0at!("<", ">"),
        "hi",
    );
}
    "# , expect ! [[r#"
                concat!
                "<>""#]] ,) ; } # [test] fn in_included () { check (r#"
//- minicore: include
//- /main.rs crate:main
include!("./included.rs");
//- /included.rs
macro_rules! foo {
    () => { fn item() {} };
}
foo$0!();
"# , expect ! [[r#"
                foo!
                fn item(){}"#]] ,) ; } # [test] fn include () { check (r#"
//- minicore: include
//- /main.rs crate:main
include$0!("./included.rs");
//- /included.rs
macro_rules! foo {
    () => { fn item() {} };
}
foo();
"# , expect ! [[r#"
                include!
                macro_rules! foo {
                    () => {
                        fn item(){}

                    };
                }
                foo();"#]] ,) ; } # [test] fn works_in_sig () { check (r#"
macro_rules! foo {
    () => { u32 };
}
fn foo() -> foo$0!() {
    42
}
"# , expect ! [[r#"
                foo!
                u32"#]] ,) ; check (r#"
macro_rules! foo {
    () => { u32 };
}
fn foo(_: foo$0!() ) {}
"# , expect ! [[r#"
                foo!
                u32"#]] ,) ; } # [test] fn works_in_generics () { check (r#"
trait Trait {}
macro_rules! foo {
    () => { Trait };
}
impl<const C: foo$0!()> Trait for () {}
"# , expect ! [[r#"
                foo!
                Trait"#]] ,) ; } # [test] fn works_in_fields () { check (r#"
macro_rules! foo {
    () => { u32 };
}
struct S {
    field: foo$0!(),
}
"# , expect ! [[r#"
                foo!
                u32"#]] ,) ; } }
};
}
