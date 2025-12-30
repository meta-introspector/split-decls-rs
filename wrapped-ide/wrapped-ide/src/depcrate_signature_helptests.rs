// Generated macro for tests (module)
macro_rules! Depcrate_signature_helptests {
() => {
// Module: crate::signature_help
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use expect_test :: { Expect , expect } ; use ide_db :: FilePosition ; use stdx :: format_to ; use test_fixture :: ChangeFixture ; use crate :: RootDatabase ; # [doc = " Creates analysis from a multi-file fixture, returns positions marked with $0."] pub (crate) fn position (# [rust_analyzer :: rust_fixture] ra_fixture : & str ,) -> (RootDatabase , FilePosition) { let mut database = RootDatabase :: default () ; let change_fixture = ChangeFixture :: parse (& database , ra_fixture) ; database . apply_change (change_fixture . change) ; let (file_id , range_or_offset) = change_fixture . file_position . expect ("expected a marker ($0)") ; let offset = range_or_offset . expect_offset () ; let position = FilePosition { file_id : file_id . file_id (& database) , offset } ; (database , position) } # [track_caller] fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let (db , position) = position (ra_fixture) ; let sig_help = hir :: attach_db (& db , | | crate :: signature_help :: signature_help (& db , position)) ; let actual = match sig_help { Some (sig_help) => { let mut rendered = String :: new () ; if let Some (docs) = & sig_help . doc { format_to ! (rendered , "{}\n------\n" , docs . as_str ()) ; } format_to ! (rendered , "{}\n" , sig_help . signature) ; let mut offset = 0 ; for (i , range) in sig_help . parameter_ranges () . iter () . enumerate () { let is_active = sig_help . active_parameter == Some (i) ; let start = u32 :: from (range . start ()) ; let gap = start . checked_sub (offset) . unwrap_or_else (| | { panic ! ("parameter ranges out of order: {:?}" , sig_help . parameter_ranges ()) }) ; rendered . extend (std :: iter :: repeat_n (' ' , gap as usize)) ; let param_text = & sig_help . signature [* range] ; let width = param_text . chars () . count () ; let marker = if is_active { '^' } else { '-' } ; rendered . extend (std :: iter :: repeat_n (marker , width)) ; offset += gap + u32 :: from (range . len ()) ; } if ! sig_help . parameter_ranges () . is_empty () { format_to ! (rendered , "\n") ; } rendered } None => String :: new () , } ; expect . assert_eq (& actual) ; } # [test] fn test_fn_signature_two_args () { check (r#"
//- minicore: sized, fn
fn foo(x: u32, y: u32) -> u32 {x + y}
fn bar() { foo($03, ); }
"# , expect ! [[r#"
                fn foo(x: u32, y: u32) -> u32
                       ^^^^^^  ------
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn foo(x: u32, y: u32) -> u32 {x + y}
fn bar() { foo(3$0, ); }
"# , expect ! [[r#"
                fn foo(x: u32, y: u32) -> u32
                       ^^^^^^  ------
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn foo(x: u32, y: u32) -> u32 {x + y}
fn bar() { foo(3,$0 ); }
"# , expect ! [[r#"
                fn foo(x: u32, y: u32) -> u32
                       ------  ^^^^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn foo(x: u32, y: u32) -> u32 {x + y}
fn bar() { foo(3, $0); }
"# , expect ! [[r#"
                fn foo(x: u32, y: u32) -> u32
                       ------  ^^^^^^
            "#]] ,) ; } # [test] fn test_fn_signature_two_args_empty () { check (r#"
//- minicore: sized, fn
fn foo(x: u32, y: u32) -> u32 {x + y}
fn bar() { foo($0); }
"# , expect ! [[r#"
                fn foo(x: u32, y: u32) -> u32
                       ^^^^^^  ------
            "#]] ,) ; } # [test] fn test_fn_signature_two_args_first_generics () { check (r#"
//- minicore: sized, fn
fn foo<T, U: Copy + Display>(x: T, y: U) -> u32
    where T: Copy + Display, U: Debug
{ x + y }

fn bar() { foo($03, ); }
"# , expect ! [[r#"
                fn foo<T, U>(x: i32, y: U) -> u32
                             ^^^^^^  ----
            "#]] ,) ; } # [test] fn test_fn_signature_no_params () { check (r#"
//- minicore: sized, fn
fn foo<T>() -> T where T: Copy + Display {}
fn bar() { foo($0); }
"# , expect ! [[r#"
                fn foo<T>() -> T
            "#]] ,) ; } # [test] fn test_fn_signature_for_impl () { check (r#"
//- minicore: sized, fn
struct F;
impl F { pub fn new() { } }
fn bar() {
    let _ : F = F::new($0);
}
"# , expect ! [[r#"
                fn new()
            "#]] ,) ; } # [test] fn test_fn_signature_for_method_self () { check (r#"
//- minicore: sized, fn
struct S;
impl S { pub fn do_it(&self) {} }

fn bar() {
    let s: S = S;
    s.do_it($0);
}
"# , expect ! [[r#"
                fn do_it(&self)
            "#]] ,) ; } # [test] fn test_fn_signature_for_method_with_arg () { check (r#"
//- minicore: sized, fn
struct S;
impl S {
    fn foo(&self, x: i32) {}
}

fn main() { S.foo($0); }
"# , expect ! [[r#"
                fn foo(&self, x: i32)
                              ^^^^^^
            "#]] ,) ; } # [test] fn test_fn_signature_for_generic_method () { check (r#"
//- minicore: sized, fn
struct S<T>(T);
impl<T> S<T> {
    fn foo(&self, x: T) {}
}

fn main() { S(1u32).foo($0); }
"# , expect ! [[r#"
                fn foo(&self, x: u32)
                              ^^^^^^
            "#]] ,) ; } # [test] fn test_fn_signature_for_method_with_arg_as_assoc_fn () { check (r#"
//- minicore: sized, fn
struct S;
impl S {
    fn foo(&self, x: i32) {}
}

fn main() { S::foo($0); }
"# , expect ! [[r#"
                fn foo(self: &S, x: i32)
                       ^^^^^^^^  ------
            "#]] ,) ; } # [test] fn test_fn_signature_with_docs_simple () { check (r#"
//- minicore: sized, fn
/// test
// non-doc-comment
fn foo(j: u32) -> u32 {
    j
}

fn bar() {
    let _ = foo($0);
}
"# , expect ! [[r#"
                test
                ------
                fn foo(j: u32) -> u32
                       ^^^^^^
            "#]] ,) ; } # [test] fn test_fn_signature_with_docs () { check (r#"
//- minicore: sized, fn
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn r#do() {
    add_one($0
}"# , expect ! [[r##"
                Adds one to the number given.

                # Examples

                ```
                let five = 5;

                assert_eq!(6, my_crate::add_one(5));
                ```
                ------
                fn add_one(x: i32) -> i32
                           ^^^^^^
            "##]] ,) ; } # [test] fn test_fn_signature_with_docs_impl () { check (r#"
//- minicore: sized, fn
struct addr;
impl addr {
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let five = 5;
    ///
    /// assert_eq!(6, my_crate::add_one(5));
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
}

pub fn do_it() {
    addr {};
    addr::add_one($0);
}
"# , expect ! [[r##"
                Adds one to the number given.

                # Examples

                ```
                let five = 5;

                assert_eq!(6, my_crate::add_one(5));
                ```
                ------
                fn add_one(x: i32) -> i32
                           ^^^^^^
            "##]] ,) ; } # [test] fn test_fn_signature_with_docs_from_actix () { check (r#"
//- minicore: sized, fn
trait Actor {
    /// Actor execution context type
    type Context;
}
trait WriteHandler<E>
where
    Self: Actor
{
    /// Method is called when writer finishes.
    ///
    /// By default this method stops actor's `Context`.
    fn finished(&mut self, ctx: &mut Self::Context) {}
}

fn foo(mut r: impl WriteHandler<()>) {
    r.finished($0);
}
"# , expect ! [[r#"
                Method is called when writer finishes.

                By default this method stops actor's `Context`.
                ------
                fn finished(&mut self, ctx: &mut <impl WriteHandler<()> as Actor>::Context)
                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            "#]] ,) ; } # [test] fn call_info_bad_offset () { check (r#"
//- minicore: sized, fn
fn foo(x: u32, y: u32) -> u32 {x + y}
fn bar() { foo $0 (3, ); }
"# , expect ! [[""]] ,) ; } # [test] fn outside_of_arg_list () { check (r#"
//- minicore: sized, fn
fn foo(a: u8) {}
fn f() {
    foo(123)$0
}
"# , expect ! [[]] ,) ; check (r#"
//- minicore: sized, fn
fn foo<T>(a: u8) {}
fn f() {
    foo::<u32>$0()
}
"# , expect ! [[]] ,) ; check (r#"
//- minicore: sized, fn
fn foo(a: u8) -> u8 {a}
fn bar(a: u8) -> u8 {a}
fn f() {
    foo(bar(123)$0)
}
"# , expect ! [[r#"
                fn foo(a: u8) -> u8
                       ^^^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
struct Vec<T>(T);
struct Vec2<T>(T);
fn f() {
    let _: Vec2<Vec<u8>$0>
}
"# , expect ! [[r#"
                struct Vec2<T>
                            ^
            "#]] ,) ; } # [test] fn test_nested_method_in_lambda () { check (r#"
//- minicore: sized, fn
struct Foo;
impl Foo { fn bar(&self, _: u32) { } }

fn bar(_: u32) { }

fn main() {
    let foo = Foo;
    std::thread::spawn(move || foo.bar($0));
}
"# , expect ! [[r#"
                fn bar(&self, _: u32)
                              ^^^^^^
            "#]] ,) ; } # [test] fn works_for_tuple_structs () { check (r#"
//- minicore: sized, fn
/// A cool tuple struct
struct S(u32, i32);
fn main() {
    let s = S(0, $0);
}
"# , expect ! [[r#"
                A cool tuple struct
                ------
                struct S(u32, i32)
                         ---  ^^^
            "#]] ,) ; } # [test] fn tuple_struct_pat () { check (r#"
//- minicore: sized, fn
/// A cool tuple struct
struct S(u32, i32);
fn main() {
    let S(0, $0);
}
"# , expect ! [[r#"
                A cool tuple struct
                ------
                struct S (u32, i32)
                          ---  ^^^
            "#]] ,) ; } # [test] fn tuple_struct_pat_rest () { check (r#"
//- minicore: sized, fn
/// A cool tuple struct
struct S(u32, i32, f32, u16);
fn main() {
    let S(0, .., $0);
}
"# , expect ! [[r#"
                A cool tuple struct
                ------
                struct S (u32, i32, f32, u16)
                          ---  ---  ---  ^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
/// A cool tuple struct
struct S(u32, i32, f32, u16, u8);
fn main() {
    let S(0, .., $0, 0);
}
"# , expect ! [[r#"
                A cool tuple struct
                ------
                struct S (u32, i32, f32, u16, u8)
                          ---  ---  ---  ^^^  --
            "#]] ,) ; check (r#"
//- minicore: sized, fn
/// A cool tuple struct
struct S(u32, i32, f32, u16);
fn main() {
    let S($0, .., 1);
}
"# , expect ! [[r#"
                A cool tuple struct
                ------
                struct S (u32, i32, f32, u16)
                          ^^^  ---  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
/// A cool tuple struct
struct S(u32, i32, f32, u16, u8);
fn main() {
    let S(1, .., 1, $0, 2);
}
"# , expect ! [[r#"
                A cool tuple struct
                ------
                struct S (u32, i32, f32, u16, u8)
                          ---  ---  ---  ^^^  --
            "#]] ,) ; check (r#"
//- minicore: sized, fn
/// A cool tuple struct
struct S(u32, i32, f32, u16);
fn main() {
    let S(1, $0.., 1);
}
"# , expect ! [[r#"
                A cool tuple struct
                ------
                struct S (u32, i32, f32, u16)
                          ---  ^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
/// A cool tuple struct
struct S(u32, i32, f32, u16);
fn main() {
    let S(1, ..$0, 1);
}
"# , expect ! [[r#"
                A cool tuple struct
                ------
                struct S (u32, i32, f32, u16)
                          ---  ^^^  ---  ---
            "#]] ,) ; } # [test] fn generic_struct () { check (r#"
//- minicore: sized, fn
struct S<T>(T);
fn main() {
    let s = S($0);
}
"# , expect ! [[r#"
                struct S<T>({unknown})
                            ^^^^^^^^^
            "#]] ,) ; } # [test] fn works_for_enum_variants () { check (r#"
//- minicore: sized, fn
enum E {
    /// A Variant
    A(i32),
    /// Another
    B,
    /// And C
    C { a: i32, b: i32 }
}

fn main() {
    let a = E::A($0);
}
"# , expect ! [[r#"
                A Variant
                ------
                enum E::A(i32)
                          ^^^
            "#]] ,) ; } # [test] fn cant_call_struct_record () { check (r#"
//- minicore: sized, fn
struct S { x: u32, y: i32 }
fn main() {
    let s = S($0);
}
"# , expect ! [[""]] ,) ; } # [test] fn cant_call_enum_record () { check (r#"
//- minicore: sized, fn
enum E {
    /// A Variant
    A(i32),
    /// Another
    B,
    /// And C
    C { a: i32, b: i32 }
}

fn main() {
    let a = E::C($0);
}
"# , expect ! [[""]] ,) ; } # [test] fn fn_signature_for_call_in_macro () { check (r#"
//- minicore: sized, fn
macro_rules! id { ($($tt:tt)*) => { $($tt)* } }
fn foo() { }
id! {
    fn bar() { foo($0); }
}
"# , expect ! [[r#"
                fn foo()
            "#]] ,) ; } # [test] fn fn_signature_for_method_call_defined_in_macro () { check (r#"
//- minicore: sized, fn
macro_rules! id { ($($tt:tt)*) => { $($tt)* } }
struct S;
id! {
    impl S {
        fn foo<'a>(&'a mut self) {}
    }
}
fn test() { S.foo($0); }
"# , expect ! [[r#"
                fn foo<'a>(&'a mut self)
            "#]] ,) ; } # [test] fn call_info_for_lambdas () { check (r#"
//- minicore: sized, fn
struct S;
fn foo(s: S) -> i32 { 92 }
fn main() {
    let _move = S;
    (|s| {{_move}; foo(s)})($0)
}
        "# , expect ! [[r#"
                impl FnOnce(s: S) -> i32
                            ^^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
struct S;
fn foo(s: S) -> i32 { 92 }
fn main() {
    (|s| foo(s))($0)
}
        "# , expect ! [[r#"
                impl Fn(s: S) -> i32
                        ^^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
struct S;
fn foo(s: S) -> i32 { 92 }
fn main() {
    let mut mutate = 0;
    (|s| { mutate = 1; foo(s) })($0)
}
        "# , expect ! [[r#"
                impl FnMut(s: S) -> i32
                           ^^^^
            "#]] ,) ; } # [test] fn call_info_for_fn_def_over_reference () { check (r#"
//- minicore: sized, fn
struct S;
fn foo(s: S) -> i32 { 92 }
fn main() {
    let bar = &&&&&foo;
    bar($0);
}
        "# , expect ! [[r#"
                fn foo(s: S) -> i32
                       ^^^^
            "#]] ,) } # [test] fn call_info_for_fn_ptr () { check (r#"
//- minicore: sized, fn
fn main(f: fn(i32, f64) -> char) {
    f(0, $0)
}
        "# , expect ! [[r#"
                fn(i32, f64) -> char
                   ---  ^^^
            "#]] ,) } # [test] fn call_info_for_fn_impl () { check (r#"
//- minicore: sized, fn
struct S;
impl core::ops::FnOnce<(i32, f64)> for S {
    type Output = char;
}
impl core::ops::FnMut<(i32, f64)> for S {}
impl core::ops::Fn<(i32, f64)> for S {}
fn main() {
    S($0);
}
        "# , expect ! [[r#"
                <S as Fn>::call(i32, f64) -> char
                                ^^^  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
struct S;
impl core::ops::FnOnce<(i32, f64)> for S {
    type Output = char;
}
impl core::ops::FnMut<(i32, f64)> for S {}
impl core::ops::Fn<(i32, f64)> for S {}
fn main() {
    S(1, $0);
}
        "# , expect ! [[r#"
                <S as Fn>::call(i32, f64) -> char
                                ---  ^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
struct S;
impl core::ops::FnOnce<(i32, f64)> for S {
    type Output = char;
}
impl core::ops::FnOnce<(char, char)> for S {
    type Output = f64;
}
fn main() {
    S($0);
}
        "# , expect ! [""] ,) ; check (r#"
//- minicore: sized, fn
struct S;
impl core::ops::FnOnce<(i32, f64)> for S {
    type Output = char;
}
impl core::ops::FnOnce<(char, char)> for S {
    type Output = f64;
}
fn main() {
    // FIXME: The ide layer loses the calling info here so we get an ambiguous trait solve result
    S(0i32, $0);
}
        "# , expect ! [""] ,) ; } # [test] fn call_info_for_unclosed_call () { check (r#"
//- minicore: sized, fn
fn foo(foo: u32, bar: u32) {}
fn main() {
    foo($0
}"# , expect ! [[r#"
                fn foo(foo: u32, bar: u32)
                       ^^^^^^^^  --------
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn foo(foo: u32, bar: u32) {}
fn main() {
    foo( $0
}"# , expect ! [[r#"
                fn foo(foo: u32, bar: u32)
                       ^^^^^^^^  --------
            "#]] ,) } # [test] fn test_multiline_argument () { check (r#"
//- minicore: sized, fn
fn callee(a: u8, b: u8) {}
fn main() {
    callee(match 0 {
        0 => 1,$0
    })
}"# , expect ! [[r#""#]] ,) ; check (r#"
//- minicore: sized, fn
fn callee(a: u8, b: u8) {}
fn main() {
    callee(match 0 {
        0 => 1,
    },$0)
}"# , expect ! [[r#"
                fn callee(a: u8, b: u8)
                          -----  ^^^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn callee(a: u8, b: u8) {}
fn main() {
    callee($0match 0 {
        0 => 1,
    })
}"# , expect ! [[r#"
                fn callee(a: u8, b: u8)
                          ^^^^^  -----
            "#]] ,) ; } # [test] fn test_generics_simple () { check (r#"
//- minicore: sized, fn
/// Option docs.
enum Option<T> {
    Some(T),
    None,
}

fn f() {
    let opt: Option<$0
}
        "# , expect ! [[r#"
                Option docs.
                ------
                enum Option<T>
                            ^
            "#]] ,) ; } # [test] fn test_generics_on_variant () { check (r#"
//- minicore: sized, fn
/// Option docs.
enum Option<T> {
    /// Some docs.
    Some(T),
    /// None docs.
    None,
}

use Option::*;

fn f() {
    None::<$0
}
        "# , expect ! [[r#"
                None docs.
                ------
                enum Option<T>
                            ^
            "#]] ,) ; } # [test] fn test_lots_of_generics () { check (r#"
//- minicore: sized, fn
trait Tr<T> {}

struct S<T>(T);

impl<T> S<T> {
    fn f<G, H>(g: G, h: impl Tr<G>) where G: Tr<()> {}
}

fn f() {
    S::<u8>::f::<(), $0
}
        "# , expect ! [[r#"
                fn f<G: Tr<()>, H>
                     ---------  ^
            "#]] ,) ; } # [test] fn test_generics_in_trait_ufcs () { check (r#"
//- minicore: sized, fn
trait Tr {
    fn f<T: Tr, U>() {}
}

struct S;

impl Tr for S {}

fn f() {
    <S as Tr>::f::<$0
}
        "# , expect ! [[r#"
                fn f<T: Tr, U>
                     ^^^^^  -
            "#]] ,) ; } # [test] fn test_generics_in_method_call () { check (r#"
//- minicore: sized, fn
struct S;

impl S {
    fn f<T>(&self) {}
}

fn f() {
    S.f::<$0
}
        "# , expect ! [[r#"
                fn f<T>
                     ^
            "#]] ,) ; } # [test] fn test_generic_param_in_method_call () { check (r#"
//- minicore: sized, fn
struct Foo;
impl Foo {
    fn test<V>(&mut self, val: V) {}
}
fn sup() {
    Foo.test($0)
}
"# , expect ! [[r#"
                fn test<V>(&mut self, val: V)
                                      ^^^^^^
            "#]] ,) ; } # [test] fn test_generic_kinds () { check (r#"
//- minicore: sized, fn
fn callee<'a, const A: u8, T, const C: u8>() {}

fn f() {
    callee::<'static, $0
}
        "# , expect ! [[r#"
                fn callee<'a, const A: u8, T, const C: u8>
                          --  ^^^^^^^^^^^  -  -----------
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn callee<'a, const A: u8, T, const C: u8>() {}

fn f() {
    callee::<NON_LIFETIME$0
}
        "# , expect ! [[r#"
                fn callee<'a, const A: u8, T, const C: u8>
                          --  ^^^^^^^^^^^  -  -----------
            "#]] ,) ; } # [test] fn test_trait_assoc_types () { check (r#"
//- minicore: sized, fn
trait Trait<'a, T> {
    type Assoc;
}
fn f() -> impl Trait<(), $0
            "# , expect ! [[r#"
                trait Trait<'a, T, Assoc = …>
                            --  -  ^^^^^^^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
trait Iterator {
    type Item;
}
fn f() -> impl Iterator<$0
            "# , expect ! [[r#"
                trait Iterator<Item = …>
                               ^^^^^^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
trait Iterator {
    type Item;
}
fn f() -> impl Iterator<Item = $0
            "# , expect ! [[r#"
                trait Iterator<Item = …>
                               ^^^^^^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
trait Tr {
    type A;
    type B;
}
fn f() -> impl Tr<$0
            "# , expect ! [[r#"
                trait Tr<A = …, B = …>
                         ^^^^^  -----
            "#]] ,) ; check (r#"
//- minicore: sized, fn
trait Tr {
    type A;
    type B;
}
fn f() -> impl Tr<B$0
            "# , expect ! [[r#"
                trait Tr<A = …, B = …>
                         ^^^^^  -----
            "#]] ,) ; check (r#"
//- minicore: sized, fn
trait Tr {
    type A;
    type B;
}
fn f() -> impl Tr<B = $0
            "# , expect ! [[r#"
                trait Tr<B = …, A = …>
                         ^^^^^  -----
            "#]] ,) ; check (r#"
//- minicore: sized, fn
trait Tr {
    type A;
    type B;
}
fn f() -> impl Tr<B = (), $0
            "# , expect ! [[r#"
                trait Tr<B = …, A = …>
                         -----  ^^^^^
            "#]] ,) ; } # [test] fn test_supertrait_assoc () { check (r#"
//- minicore: sized, fn
trait Super {
    type SuperTy;
}
trait Sub: Super + Super {
    type SubTy;
}
fn f() -> impl Sub<$0
            "# , expect ! [[r#"
                trait Sub<SubTy = …, SuperTy = …>
                          ^^^^^^^^^  -----------
            "#]] ,) ; } # [test] fn no_assoc_types_outside_type_bounds () { check (r#"
//- minicore: sized, fn
trait Tr<T> {
    type Assoc;
}

impl Tr<$0
        "# , expect ! [[r#"
            trait Tr<T>
                     ^
        "#]] ,) ; } # [test] fn impl_trait () { check (r#"
//- minicore: sized, fn
trait Trait<T> {}
struct Wrap<T>(T);
fn foo<U>(x: Wrap<impl Trait<U>>) {}
fn f() {
    foo::<i8>($0)
}
"# , expect ! [[r#"
                fn foo<U>(x: Wrap<impl Trait<U>>)
                          ^^^^^^^^^^^^^^^^^^^^^^
            "#]] ,) ; } # [test] fn fully_qualified_syntax () { check (r#"
//- minicore: sized, fn
fn f() {
    trait A { fn foo(&self, other: Self); }
    A::foo(&self$0, other);
}
"# , expect ! [[r#"
                fn foo(self: &Self, other: Self)
                       ^^^^^^^^^^^  -----------
            "#]] ,) ; } # [test] fn help_for_generic_call () { check (r#"
//- minicore: sized, fn
fn f<F: FnOnce(u8, u16) -> i32>(f: F) {
    f($0)
}
"# , expect ! [[r#"
                impl FnOnce(u8, u16) -> i32
                            ^^  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn f<T, F: FnMut(&T, u16) -> &T>(f: F) {
    f($0)
}
"# , expect ! [[r#"
                impl FnMut(&T, u16) -> &T
                           ^^  ---
            "#]] ,) ; } # [test] fn regression_13579 () { check (r#"
//- minicore: sized, fn
fn f() {
    take(2)($0);
}

fn take<C, Error>(
    count: C
) -> impl Fn() -> C  {
    move || count
}
"# , expect ! [""] ,) ; } # [test] fn record_literal () { check (r#"
//- minicore: sized, fn
struct Strukt<T, U = ()> {
    t: T,
    u: U,
    unit: (),
}
fn f() {
    Strukt {
        u: 0,
        $0
    }
}
"# , expect ! [[r#"
                struct Strukt { u: i32, t: T, unit: () }
                                ------  ^^^^  --------
            "#]] ,) ; } # [test] fn record_literal_nonexistent_field () { check (r#"
//- minicore: sized, fn
struct Strukt {
    a: u8,
}
fn f() {
    Strukt {
        b: 8,
        $0
    }
}
"# , expect ! [[r#"
                struct Strukt { a: u8 }
                                -----
            "#]] ,) ; } # [test] fn tuple_variant_record_literal () { check (r#"
//- minicore: sized, fn
enum Opt {
    Some(u8),
}
fn f() {
    Opt::Some {$0}
}
"# , expect ! [[r#"
                enum Opt::Some { 0: u8 }
                                 ^^^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
enum Opt {
    Some(u8),
}
fn f() {
    Opt::Some {0:0,$0}
}
"# , expect ! [[r#"
                enum Opt::Some { 0: u8 }
                                 -----
            "#]] ,) ; } # [test] fn record_literal_self () { check (r#"
//- minicore: sized, fn
struct S { t: u8 }
impl S {
    fn new() -> Self {
        Self { $0 }
    }
}
        "# , expect ! [[r#"
                struct S { t: u8 }
                           ^^^^^
            "#]] ,) ; } # [test] fn record_pat () { check (r#"
//- minicore: sized, fn
struct Strukt<T, U = ()> {
    t: T,
    u: U,
    unit: (),
}
fn f() {
    let Strukt {
        u: 0,
        $0
    }
}
"# , expect ! [[r#"
                struct Strukt { u: i32, t: T, unit: () }
                                ------  ^^^^  --------
            "#]] ,) ; } # [test] fn test_enum_in_nested_method_in_lambda () { check (r#"
//- minicore: sized, fn
enum A {
    A,
    B
}

fn bar(_: A) { }

fn main() {
    let foo = Foo;
    std::thread::spawn(move || { bar(A:$0) } );
}
"# , expect ! [[r#"
                fn bar(_: A)
                       ^^^^
            "#]] ,) ; } # [test] fn test_tuple_expr_free () { check (r#"
//- minicore: sized, fn
fn main() {
    (0$0, 1, 3);
}
"# , expect ! [[r#"
                (i32, i32, i32)
                 ^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    ($0 1, 3);
}
"# , expect ! [[r#"
                (i32, i32)
                 ^^^  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    (1, 3 $0);
}
"# , expect ! [[r#"
                (i32, i32)
                 ---  ^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    (1, 3 $0,);
}
"# , expect ! [[r#"
                (i32, i32)
                 ---  ^^^
            "#]] ,) ; } # [test] fn test_tuple_expr_expected () { check (r#"
//- minicore: sized, fn
fn main() {
    let _: (&str, u32, u32)= ($0, 1, 3);
}
"# , expect ! [[r#"
                (&str, u32, u32)
                 ^^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let _: (&str, u32, u32, u32) = ($0, 1, 3);
}
"# , expect ! [[r#"
                (&str, u32, u32)
                 ^^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let _: (&str, u32, u32)= ($0, 1, 3, 5);
}
"# , expect ! [[r#"
                (&str, u32, u32, i32)
                 ^^^^  ---  ---  ---
            "#]] ,) ; } # [test] fn test_tuple_pat_free () { check (r#"
//- minicore: sized, fn
fn main() {
    let ($0, 1, 3);
}
"# , expect ! [[r#"
                ({unknown}, i32, i32)
                 ^^^^^^^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (0$0, 1, 3);
}
"# , expect ! [[r#"
                (i32, i32, i32)
                 ^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let ($0 1, 3);
}
"# , expect ! [[r#"
                (i32, i32)
                 ^^^  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (1, 3 $0);
}
"# , expect ! [[r#"
                (i32, i32)
                 ---  ^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (1, 3 $0,);
}
"# , expect ! [[r#"
                (i32, i32)
                 ---  ^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (1, 3 $0, ..);
}
"# , expect ! [[r#"
                (i32, i32)
                 ---  ^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (1, 3, .., $0);
}
"# , expect ! [[r#"
                (i32, i32)
                 ---  ^^^
            "#]] ,) ; } # [test] fn test_tuple_pat_expected () { check (r#"
//- minicore: sized, fn
fn main() {
    let (0$0, 1, 3): (i32, i32, i32);
}
"# , expect ! [[r#"
                (i32, i32, i32)
                 ^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let ($0, 1, 3): (i32, i32, i32);
}
"# , expect ! [[r#"
                (i32, i32, i32)
                 ^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (1, 3 $0): (i32,);
}
"# , expect ! [[r#"
                (i32, i32)
                 ---  ^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (1, 3 $0, ..): (i32, i32, i32, i32);
}
"# , expect ! [[r#"
                (i32, i32, i32, i32)
                 ---  ^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (1, 3, .., $0): (i32, i32, i32);
}
"# , expect ! [[r#"
                (i32, i32, i32)
                 ---  ---  ^^^
            "#]] ,) ; } # [test] fn test_tuple_pat_expected_inferred () { check (r#"
//- minicore: sized, fn
fn main() {
    let (0$0, 1, 3) = (1, 2 ,3);
}
"# , expect ! [[r#"
                (i32, i32, i32)
                 ^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let ($0 1, 3) = (1, 2, 3);
}
"# , expect ! [[r#"
                (i32, i32)
                 ^^^  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (1, 3 $0) = (1,);
}
"# , expect ! [[r#"
                (i32, i32)
                 ---  ^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (1, 3 $0, ..) = (1, 2, 3, 4);
}
"# , expect ! [[r#"
                (i32, i32, i32, i32)
                 ---  ^^^  ---  ---
            "#]] ,) ; check (r#"
//- minicore: sized, fn
fn main() {
    let (1, 3, .., $0) = (1, 2, 3);
}
"# , expect ! [[r#"
                (i32, i32, i32)
                 ---  ---  ^^^
            "#]] ,) ; } # [test] fn test_tuple_generic_param () { check (r#"
//- minicore: sized, fn
struct S<T>(T);

fn main() {
    let s: S<$0
}
            "# , expect ! [[r#"
                struct S<T>
                         ^
            "#]] ,) ; } # [test] fn test_enum_generic_param () { check (r#"
//- minicore: sized, fn
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let opt: Option<$0
}
            "# , expect ! [[r#"
                enum Option<T>
                            ^
            "#]] ,) ; } # [test] fn test_enum_variant_generic_param () { check (r#"
//- minicore: sized, fn
enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let opt = Option::Some($0);
}
            "# , expect ! [[r#"
                enum Option<T>::Some({unknown})
                                     ^^^^^^^^^
            "#]] ,) ; } # [test] fn test_generic_arg_with_default () { check (r#"
//- minicore: sized, fn
struct S<T = u8> {
    field: T,
}

fn main() {
    let s: S<$0
}
            "# , expect ! [[r#"
                struct S<T = u8>
                         ^^^^^^
            "#]] ,) ; check (r#"
//- minicore: sized, fn
struct S<const C: u8 = 5> {
    field: C,
}

fn main() {
    let s: S<$0
}
            "# , expect ! [[r#"
                struct S<const C: u8 = 5>
                         ^^^^^^^^^^^^^^^
            "#]] ,) ; } # [test] fn test_async_function () { check (r#"
//- minicore: sized, fn, future, result
pub async fn conn_mut<F, T>(f: F) -> Result<T, i32>
where
    F: FnOnce() -> T,
{
    Ok(f())
}

fn main() {
    conn_mut($0)
}
            "# , expect ! [[r#"
                async fn conn_mut<F: FnOnce() -> T, T>(f: F) -> Result<T, i32>
                                                       ^^^^
            "#]] ,) ; } }
};
}
