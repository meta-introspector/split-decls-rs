// Generated macro for tests (module)
macro_rules! Depcrate_typingtests {
() => {
// Module: crate::typing
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use test_utils :: { assert_eq_text , extract_offset } ; use super :: * ; impl ExtendedTextEdit { fn apply (& self , text : & mut String) { self . edit . apply (text) ; } } fn do_type_char (char_typed : char , before : & str) -> Option < String > { let (offset , mut before) = extract_offset (before) ; let edit = TextEdit :: insert (offset , char_typed . to_string ()) ; edit . apply (& mut before) ; let parse = SourceFile :: parse (& before , span :: Edition :: CURRENT_FIXME) ; on_char_typed_ (& parse , offset , char_typed , span :: Edition :: CURRENT_FIXME) . map (| it | { it . apply (& mut before) ; before . to_string () }) } fn type_char (char_typed : char , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let actual = do_type_char (char_typed , ra_fixture_before) . unwrap_or_else (| | panic ! ("typing `{char_typed}` did nothing")) ; assert_eq_text ! (ra_fixture_after , & actual) ; } fn type_char_noop (char_typed : char , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str) { let file_change = do_type_char (char_typed , ra_fixture_before) ; assert_eq ! (file_change , None) } # [test] fn test_semi_after_let () { type_char_noop ('=' , r"
fn foo() {
    let foo =$0
}
" ,) ; type_char ('=' , r#"
fn foo() {
    let foo $0 1 + 1
}
"# , r#"
fn foo() {
    let foo = 1 + 1;
}
"# ,) ; type_char_noop ('=' , r#"
fn foo() {
    let difference $0(counts: &HashMap<(char, char), u64>, last: char) -> u64 {
        // ...
    }
}
"# ,) ; type_char_noop ('=' , r"
fn foo() {
    let foo =$0
    let bar = 1;
}
" ,) ; type_char_noop ('=' , r"
fn foo() {
    let foo =$0
     1 + 1
}
" ,) ; } # [test] fn test_semi_after_assign () { type_char ('=' , r#"
fn f() {
    i $0 0
}
"# , r#"
fn f() {
    i = 0;
}
"# ,) ; type_char ('=' , r#"
fn f() {
    i $0 0
    i
}
"# , r#"
fn f() {
    i = 0;
    i
}
"# ,) ; type_char_noop ('=' , r#"
fn f(x: u8) {
    if x $0
}
"# ,) ; type_char_noop ('=' , r#"
fn f(x: u8) {
    if x $0 {}
}
"# ,) ; type_char_noop ('=' , r#"
fn f(x: u8) {
    if x $0 0 {}
}
"# ,) ; type_char_noop ('=' , r#"
fn f() {
    g(i $0 0);
}
"# ,) ; } # [test] fn assign_to_eq () { type_char ('=' , r#"
fn f(a: u8) {
    a =$0 0;
}
"# , r#"
fn f(a: u8) {
    a == 0
}
"# ,) ; type_char ('=' , r#"
fn f(a: u8) {
    a $0= 0;
}
"# , r#"
fn f(a: u8) {
    a == 0
}
"# ,) ; type_char_noop ('=' , r#"
fn f(a: u8) {
    let e = a =$0 0;
}
"# ,) ; type_char_noop ('=' , r#"
fn f(a: u8) {
    let e = a =$0 0;
    e
}
"# ,) ; } # [test] fn indents_new_chain_call () { type_char ('.' , r#"
fn main() {
    xs.foo()
    $0
}
            "# , r#"
fn main() {
    xs.foo()
        .
}
            "# ,) ; type_char_noop ('.' , r#"
fn main() {
    xs.foo()
        $0
}
            "# ,) } # [test] fn indents_new_chain_call_with_semi () { type_char ('.' , r"
fn main() {
    xs.foo()
    $0;
}
            " , r#"
fn main() {
    xs.foo()
        .;
}
            "# ,) ; type_char_noop ('.' , r#"
fn main() {
    xs.foo()
        $0;
}
            "# ,) } # [test] fn indents_new_chain_call_with_let () { type_char ('.' , r#"
fn main() {
    let _ = foo
    $0
    bar()
}
"# , r#"
fn main() {
    let _ = foo
        .
    bar()
}
"# ,) ; } # [test] fn indents_continued_chain_call () { type_char ('.' , r#"
fn main() {
    xs.foo()
        .first()
    $0
}
            "# , r#"
fn main() {
    xs.foo()
        .first()
        .
}
            "# ,) ; type_char_noop ('.' , r#"
fn main() {
    xs.foo()
        .first()
        $0
}
            "# ,) ; } # [test] fn indents_middle_of_chain_call () { type_char ('.' , r#"
fn source_impl() {
    let var = enum_defvariant_list().unwrap()
    $0
        .nth(92)
        .unwrap();
}
            "# , r#"
fn source_impl() {
    let var = enum_defvariant_list().unwrap()
        .
        .nth(92)
        .unwrap();
}
            "# ,) ; type_char_noop ('.' , r#"
fn source_impl() {
    let var = enum_defvariant_list().unwrap()
        $0
        .nth(92)
        .unwrap();
}
            "# ,) ; } # [test] fn dont_indent_freestanding_dot () { type_char_noop ('.' , r#"
fn main() {
    $0
}
            "# ,) ; type_char_noop ('.' , r#"
fn main() {
$0
}
            "# ,) ; } # [test] fn adds_space_after_return_type () { type_char ('>' , r#"
fn foo() -$0{ 92 }
"# , r#"
fn foo() -> { 92 }
"# ,) ; } # [test] fn adds_closing_brace_for_expr () { type_char ('{' , r#"
fn f() { match () { _ => $0() } }
            "# , r#"
fn f() { match () { _ => {()} } }
            "# ,) ; type_char ('{' , r#"
fn f() { $0() }
            "# , r#"
fn f() { {()} }
            "# ,) ; type_char ('{' , r#"
fn f() { let x = $0(); }
            "# , r#"
fn f() { let x = {()}; }
            "# ,) ; type_char ('{' , r#"
fn f() { let x = $0a.b(); }
            "# , r#"
fn f() { let x = {a.b()}; }
            "# ,) ; type_char ('{' , r#"
const S: () = $0();
fn f() {}
            "# , r#"
const S: () = {()};
fn f() {}
            "# ,) ; type_char ('{' , r#"
const S: () = $0a.b();
fn f() {}
            "# , r#"
const S: () = {a.b()};
fn f() {}
            "# ,) ; type_char ('{' , r#"
fn f() {
    match x {
        0 => $0(),
        1 => (),
    }
}
            "# , r#"
fn f() {
    match x {
        0 => {()},
        1 => (),
    }
}
            "# ,) ; type_char ('{' , r#"
fn main() {
    #[allow(unreachable_code)]
    $0g();
}
            "# , r#"
fn main() {
    #[allow(unreachable_code)]
    {g()};
}
            "# ,) ; } # [test] fn noop_in_string_literal () { type_char_noop ('{' , r##"
fn check_with(#[rust_analyzer::rust_fixture] ra_fixture: &str, expect: Expect) {
    let base = r#"
enum E { T(), R$0, C }
use self::E::X;
const Z: E = E::C;
mod m {}
asdasdasdasdasdasda
sdasdasdasdasdasda
sdasdasdasdasd
"#;
    let actual = completion_list(&format!("{}\n{}", base, ra_fixture));
    expect.assert_eq(&actual)
}
            "## ,) ; } # [test] fn noop_in_item_position_with_macro () { type_char_noop ('{' , r#"$0println!();"#) ; type_char_noop ('{' , r#"
fn main() $0println!("hello");
}"# ,) ; } # [test] fn adds_closing_brace_for_use_tree () { type_char ('{' , r#"
use some::$0Path;
            "# , r#"
use some::{Path};
            "# ,) ; type_char ('{' , r#"
use some::{Path, $0Other};
            "# , r#"
use some::{Path, {Other}};
            "# ,) ; type_char ('{' , r#"
use some::{$0Path, Other};
            "# , r#"
use some::{{Path}, Other};
            "# ,) ; type_char ('{' , r#"
use some::path::$0to::Item;
            "# , r#"
use some::path::{to::Item};
            "# ,) ; type_char ('{' , r#"
use some::$0path::to::Item;
            "# , r#"
use some::{path::to::Item};
            "# ,) ; type_char ('{' , r#"
use $0some::path::to::Item;
            "# , r#"
use {some::path::to::Item};
            "# ,) ; type_char ('{' , r#"
use some::path::$0to::{Item};
            "# , r#"
use some::path::{to::{Item}};
            "# ,) ; type_char ('{' , r#"
use $0Thing as _;
            "# , r#"
use {Thing as _};
            "# ,) ; type_char_noop ('{' , r#"
use some::pa$0th::to::Item;
            "# ,) ; } # [test] fn adds_closing_parenthesis_for_expr () { type_char ('(' , r#"
fn f() { match () { _ => $0() } }
            "# , r#"
fn f() { match () { _ => (()) } }
            "# ,) ; type_char ('(' , r#"
fn f() { $0() }
            "# , r#"
fn f() { (()) }
            "# ,) ; type_char ('(' , r#"
fn f() { let x = $0(); }
            "# , r#"
fn f() { let x = (()); }
            "# ,) ; type_char ('(' , r#"
fn f() { let x = $0a.b(); }
            "# , r#"
fn f() { let x = (a.b()); }
            "# ,) ; type_char ('(' , r#"
const S: () = $0();
fn f() {}
            "# , r#"
const S: () = (());
fn f() {}
            "# ,) ; type_char ('(' , r#"
const S: () = $0a.b();
fn f() {}
            "# , r#"
const S: () = (a.b());
fn f() {}
            "# ,) ; type_char ('(' , r#"
fn f() {
    match x {
        0 => $0(),
        1 => (),
    }
}
            "# , r#"
fn f() {
    match x {
        0 => (()),
        1 => (),
    }
}
            "# ,) ; type_char ('(' , r#"
        fn f() {
            let z = Some($03);
        }
                    "# , r#"
        fn f() {
            let z = Some((3));
        }
                    "# ,) ; } # [test] fn preceding_whitespace_is_significant_for_closing_brackets () { type_char_noop ('(' , r#"
fn f() { a.b$0if true {} }
"# ,) ; type_char_noop ('(' , r#"
fn f() { foo$0{} }
"# ,) ; } # [test] fn adds_closing_parenthesis_for_pat () { type_char ('(' , r#"
fn f() { match () { $0() => () } }
"# , r#"
fn f() { match () { (()) => () } }
"# ,) ; type_char ('(' , r#"
fn f($0n: ()) {}
"# , r#"
fn f((n): ()) {}
"# ,) ; } # [test] fn adds_closing_parenthesis_for_ty () { type_char ('(' , r#"
fn f(n: $0()) {}
"# , r#"
fn f(n: (())) {}
"# ,) ; type_char ('(' , r#"
fn f(n: $0a::b::<d>::c) {}
"# , r#"
fn f(n: (a::b::<d>::c)) {}
"# ,) ; } # [test] fn adds_closing_angles_for_ty () { type_char ('<' , r#"
fn f(n: $0()) {}
"# , r#"
fn f(n: <()>) {}
"# ,) ; type_char ('<' , r#"
fn f(n: $0a::b::<d>::c) {}
"# , r#"
fn f(n: <a::b::<d>::c>) {}
"# ,) ; type_char ('<' , r#"
fn f(n: a$0b::<d>::c) {}
"# , r#"
fn f(n: a<>b::<d>::c) {}
"# ,) ; } # [test] fn parenthesis_noop_in_string_literal () { type_char_noop ('(' , r##"
fn check_with(#[rust_analyzer::rust_fixture] ra_fixture: &str, expect: Expect) {
    let base = r#"
enum E { T(), R$0, C }
use self::E::X;
const Z: E = E::C;
mod m {}
asdasdasdasdasdasda
sdasdasdasdasdasda
sdasdasdasdasd
"#;
    let actual = completion_list(&format!("{}\n{}", base, ra_fixture));
    expect.assert_eq(&actual)
}
            "## ,) ; } # [test] fn parenthesis_noop_in_item_position_with_macro () { type_char_noop ('(' , r#"$0println!();"#) ; type_char_noop ('(' , r#"
fn main() $0println!("hello");
}"# ,) ; } # [test] fn parenthesis_noop_in_use_tree () { type_char_noop ('(' , r#"
use some::$0Path;
            "# ,) ; type_char_noop ('(' , r#"
use some::{Path, $0Other};
            "# ,) ; type_char_noop ('(' , r#"
use some::{$0Path, Other};
            "# ,) ; type_char_noop ('(' , r#"
use some::path::$0to::Item;
            "# ,) ; type_char_noop ('(' , r#"
use some::$0path::to::Item;
            "# ,) ; type_char_noop ('(' , r#"
use $0some::path::to::Item;
            "# ,) ; type_char_noop ('(' , r#"
use some::path::$0to::{Item};
            "# ,) ; type_char_noop ('(' , r#"
use $0Thing as _;
            "# ,) ; type_char_noop ('(' , r#"
use some::pa$0th::to::Item;
            "# ,) ; type_char_noop ('<' , r#"
use some::pa$0th::to::Item;
            "# ,) ; } # [test] fn adds_closing_angle_bracket_for_generic_args () { type_char ('<' , r#"
fn foo() {
    bar::$0
}
            "# , r#"
fn foo() {
    bar::<>
}
            "# ,) ; type_char ('<' , r#"
fn foo(bar: &[u64]) {
    bar.iter().collect::$0();
}
            "# , r#"
fn foo(bar: &[u64]) {
    bar.iter().collect::<>();
}
            "# ,) ; } # [test] fn adds_closing_angle_bracket_for_generic_params () { type_char ('<' , r#"
fn foo$0() {}
            "# , r#"
fn foo<>() {}
            "# ,) ; type_char ('<' , r#"
fn foo$0
            "# , r#"
fn foo<>
            "# ,) ; type_char ('<' , r#"
struct Foo$0 {}
            "# , r#"
struct Foo<> {}
            "# ,) ; type_char ('<' , r#"
struct Foo$0();
            "# , r#"
struct Foo<>();
            "# ,) ; type_char ('<' , r#"
struct Foo$0
            "# , r#"
struct Foo<>
            "# ,) ; type_char ('<' , r#"
enum Foo$0
            "# , r#"
enum Foo<>
            "# ,) ; type_char ('<' , r#"
trait Foo$0
            "# , r#"
trait Foo<>
            "# ,) ; type_char ('<' , r#"
type Foo$0 = Bar;
            "# , r#"
type Foo<> = Bar;
            "# ,) ; type_char ('<' , r#"
impl<T> Foo$0 {}
            "# , r#"
impl<T> Foo<> {}
            "# ,) ; type_char ('<' , r#"
impl Foo$0 {}
            "# , r#"
impl Foo<> {}
            "# ,) ; } # [test] fn dont_add_closing_angle_bracket_for_comparison () { type_char_noop ('<' , r#"
fn main() {
    42$0
}
            "# ,) ; type_char_noop ('<' , r#"
fn main() {
    42 $0
}
            "# ,) ; type_char_noop ('<' , r#"
fn main() {
    let foo = 42;
    foo $0
}
            "# ,) ; } # [test] fn dont_add_closing_angle_bracket_if_it_is_already_there () { type_char_noop ('<' , r#"
fn foo() {
    bar::$0>
}
            "# ,) ; type_char_noop ('<' , r#"
fn foo(bar: &[u64]) {
    bar.iter().collect::$0   >();
}
            "# ,) ; type_char_noop ('<' , r#"
fn foo$0>() {}
            "# ,) ; type_char_noop ('<' , r#"
fn foo$0>
            "# ,) ; type_char_noop ('<' , r#"
struct Foo$0> {}
            "# ,) ; type_char_noop ('<' , r#"
struct Foo$0>();
            "# ,) ; type_char_noop ('<' , r#"
struct Foo$0>
            "# ,) ; type_char_noop ('<' , r#"
enum Foo$0>
            "# ,) ; type_char_noop ('<' , r#"
trait Foo$0>
            "# ,) ; type_char_noop ('<' , r#"
type Foo$0> = Bar;
            "# ,) ; type_char_noop ('<' , r#"
impl$0> Foo {}
            "# ,) ; type_char_noop ('<' , r#"
impl<T> Foo$0> {}
            "# ,) ; type_char_noop ('<' , r#"
impl Foo$0> {}
            "# ,) ; } # [test] fn regression_629 () { type_char_noop ('.' , r#"
fn foo() {
    CompletionItem::new(
        CompletionKind::Reference,
        ctx.source_range(),
        field.name().to_string(),
    )
    .foo()
    $0
}
"# ,) ; type_char_noop ('.' , r#"
fn foo() {
    CompletionItem::new(
        CompletionKind::Reference,
        ctx.source_range(),
        field.name().to_string(),
    )
    $0
}
"# ,) ; } # [test] fn completes_pipe_param_list () { type_char ('|' , r#"
fn foo() {
    $0
}
"# , r#"
fn foo() {
    ||
}
"# ,) ; type_char ('|' , r#"
fn foo() {
    $0 a
}
"# , r#"
fn foo() {
    || a
}
"# ,) ; type_char_noop ('|' , r#"
fn foo() {
    let $0
}
"# ,) ; } # [test] fn adds_parentheses_around_trait_object_in_ref_type () { type_char ('+' , r#"
fn foo(x: &dyn A$0) {}
"# , r#"
fn foo(x: &(dyn A+)) {}
"# ,) ; type_char ('+' , r#"
fn foo(x: &'static dyn A$0B) {}
"# , r#"
fn foo(x: &'static (dyn A+B)) {}
"# ,) ; type_char_noop ('+' , r#"
fn foo(x: &(dyn A$0)) {}
"# ,) ; type_char_noop ('+' , r#"
fn foo(x: Box<dyn A$0>) {}
"# ,) ; } # [test] fn adds_parentheses_around_trait_object_in_ptr_type () { type_char ('+' , r#"
fn foo(x: *const dyn A$0) {}
"# , r#"
fn foo(x: *const (dyn A+)) {}
"# ,) ; } # [test] fn adds_parentheses_around_trait_object_in_return_type () { type_char ('+' , r#"
fn foo(x: fn() -> dyn A$0) {}
"# , r#"
fn foo(x: fn() -> (dyn A+)) {}
"# ,) ; } }
};
}
