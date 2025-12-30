// Generated macro for tests (module)
macro_rules! Depcrate_folding_rangestests {
() => {
// Module: crate::folding_ranges
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use test_utils :: extract_tags ; use super :: * ; # [track_caller] fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str) { let (ranges , text) = extract_tags (ra_fixture , "fold") ; let parse = SourceFile :: parse (& text , span :: Edition :: CURRENT) ; let mut folds = folding_ranges (& parse . tree ()) ; folds . sort_by_key (| fold | (fold . range . start () , fold . range . end ())) ; assert_eq ! (folds . len () , ranges . len () , "The amount of folds is different than the expected amount") ; for (fold , (range , attr)) in folds . iter () . zip (ranges . into_iter ()) { assert_eq ! (fold . range . start () , range . start () , "mismatched start of folding ranges") ; assert_eq ! (fold . range . end () , range . end () , "mismatched end of folding ranges") ; let kind = match fold . kind { FoldKind :: Comment => "comment" , FoldKind :: Imports => "imports" , FoldKind :: Modules => "mods" , FoldKind :: Block => "block" , FoldKind :: ArgList => "arglist" , FoldKind :: Region => "region" , FoldKind :: Consts => "consts" , FoldKind :: Statics => "statics" , FoldKind :: TypeAliases => "typealiases" , FoldKind :: Array => "array" , FoldKind :: WhereClause => "whereclause" , FoldKind :: ReturnType => "returntype" , FoldKind :: MatchArm => "matcharm" , FoldKind :: Function => "function" , FoldKind :: ExternCrates => "externcrates" , } ; assert_eq ! (kind , & attr . unwrap ()) ; } } # [test] fn test_fold_func_with_multiline_param_list () { check (r#"
<fold function>fn func<fold arglist>(
    a: i32,
    b: i32,
    c: i32,
)</fold> <fold block>{



}</fold></fold>
"# ,) ; } # [test] fn test_fold_comments () { check (r#"
<fold comment>// Hello
// this is a multiline
// comment
//</fold>

// But this is not

fn main() <fold block>{
    <fold comment>// We should
    // also
    // fold
    // this one.</fold>
    <fold comment>//! But this one is different
    //! because it has another flavor</fold>
    <fold comment>/* As does this
    multiline comment */</fold>
}</fold>
"# ,) ; } # [test] fn test_fold_imports () { check (r#"
use std::<fold block>{
    str,
    vec,
    io as iop
}</fold>;
"# ,) ; } # [test] fn test_fold_mods () { check (r#"

pub mod foo;
<fold mods>mod after_pub;
mod after_pub_next;</fold>

<fold mods>mod before_pub;
mod before_pub_next;</fold>
pub mod bar;

mod not_folding_single;
pub mod foobar;
pub not_folding_single_next;

<fold mods>#[cfg(test)]
mod with_attribute;
mod with_attribute_next;</fold>

mod inline0 {}
mod inline1 {}

mod inline2 <fold block>{

}</fold>
"# ,) ; } # [test] fn test_fold_import_groups () { check (r#"
<fold imports>use std::str;
use std::vec;
use std::io as iop;</fold>

<fold imports>use std::mem;
use std::f64;</fold>

<fold imports>use std::collections::HashMap;
// Some random comment
use std::collections::VecDeque;</fold>
"# ,) ; } # [test] fn test_fold_import_and_groups () { check (r#"
<fold imports>use std::str;
use std::vec;
use std::io as iop;</fold>

<fold imports>use std::mem;
use std::f64;</fold>

use std::collections::<fold block>{
    HashMap,
    VecDeque,
}</fold>;
// Some random comment
"# ,) ; } # [test] fn test_folds_structs () { check (r#"
struct Foo <fold block>{
}</fold>
"# ,) ; } # [test] fn test_folds_traits () { check (r#"
trait Foo <fold block>{
}</fold>
"# ,) ; } # [test] fn test_folds_macros () { check (r#"
macro_rules! foo <fold block>{
    ($($tt:tt)*) => { $($tt)* }
}</fold>
"# ,) ; } # [test] fn test_fold_match_arms () { check (r#"
fn main() <fold block>{
    match 0 <fold block>{
        0 => 0,
        _ => 1,
    }</fold>
}</fold>
"# ,) ; } # [test] fn test_fold_multiline_non_block_match_arm () { check (r#"
            fn main() <fold block>{
                match foo <fold block>{
                    block => <fold block>{
                    }</fold>,
                    matcharm => <fold matcharm>some.
                        call().
                        chain()</fold>,
                    matcharm2
                        => 0,
                    match_expr => <fold matcharm>match foo2 <fold block>{
                        bar => (),
                    }</fold></fold>,
                    array_list => <fold array>[
                        1,
                        2,
                        3,
                    ]</fold>,
                    structS => <fold matcharm>StructS <fold block>{
                        a: 31,
                    }</fold></fold>,
                }</fold>
            }</fold>
            "# ,) } # [test] fn fold_big_calls () { check (r#"
fn main() <fold block>{
    frobnicate<fold arglist>(
        1,
        2,
        3,
    )</fold>
}</fold>
"# ,) } # [test] fn fold_record_literals () { check (r#"
const _: S = S <fold block>{

}</fold>;
"# ,) } # [test] fn fold_multiline_params () { check (r#"
<fold function>fn foo<fold arglist>(
    x: i32,
    y: String,
)</fold> {}</fold>
"# ,) } # [test] fn fold_multiline_array () { check (r#"
const FOO: [usize; 4] = <fold array>[
    1,
    2,
    3,
    4,
]</fold>;
"# ,) } # [test] fn fold_region () { check (r#"
// 1. some normal comment
<fold region>// region: test
// 2. some normal comment
<fold region>// region: inner
fn f() {}
// endregion</fold>
fn f2() {}
// endregion: test</fold>
"# ,) } # [test] fn fold_consecutive_const () { check (r#"
<fold consts>const FIRST_CONST: &str = "first";
const SECOND_CONST: &str = "second";</fold>
"# ,) } # [test] fn fold_consecutive_static () { check (r#"
<fold statics>static FIRST_STATIC: &str = "first";
static SECOND_STATIC: &str = "second";</fold>
"# ,) } # [test] fn fold_where_clause () { check (r#"
fn foo()
<fold whereclause>where
    A: Foo,
    B: Foo,
    C: Foo,
    D: Foo,</fold> {}

fn bar()
<fold whereclause>where
    A: Bar,</fold> {}
"# ,) } # [test] fn fold_return_type () { check (r#"
fn foo()<fold returntype>-> (
    bool,
    bool,
)</fold> { (true, true) }

fn bar() -> (bool, bool) { (true, true) }
"# ,) } # [test] fn fold_generics () { check (r#"
type Foo<T, U> = foo<fold arglist><
    T,
    U,
></fold>;
"# ,) } # [test] fn test_fold_doc_comments_with_multiline_paramlist_function () { check (r#"
<fold comment>/// A very very very very very very very very very very very very very very very
/// very very very long description</fold>
<fold function>fn foo<fold arglist>(
    very_long_parameter_name: u32,
    another_very_long_parameter_name: u32,
    third_very_long_param: u32,
)</fold> <fold block>{
    todo!()
}</fold></fold>
"# ,) ; } }
};
}
