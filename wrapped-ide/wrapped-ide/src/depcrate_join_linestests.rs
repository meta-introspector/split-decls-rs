// Generated macro for tests (module)
macro_rules! Depcrate_join_linestests {
() => {
// Module: crate::join_lines
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use test_utils :: { add_cursor , assert_eq_text , extract_offset , extract_range } ; use super :: * ; fn check_join_lines (# [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let config = JoinLinesConfig { join_else_if : true , remove_trailing_comma : true , unwrap_trivial_blocks : true , join_assignments : true , } ; let (before_cursor_pos , before) = extract_offset (ra_fixture_before) ; let file = SourceFile :: parse (& before , span :: Edition :: CURRENT) . ok () . unwrap () ; let range = TextRange :: empty (before_cursor_pos) ; let result = join_lines (& config , & file , range) ; let actual = { let mut actual = before ; result . apply (& mut actual) ; actual } ; let actual_cursor_pos = result . apply_to_offset (before_cursor_pos) . expect ("cursor position is affected by the edit") ; let actual = add_cursor (& actual , actual_cursor_pos) ; assert_eq_text ! (ra_fixture_after , & actual) ; } fn check_join_lines_sel (# [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let config = JoinLinesConfig { join_else_if : true , remove_trailing_comma : true , unwrap_trivial_blocks : true , join_assignments : true , } ; let (sel , before) = extract_range (ra_fixture_before) ; let parse = SourceFile :: parse (& before , span :: Edition :: CURRENT) ; let result = join_lines (& config , & parse . tree () , sel) ; let actual = { let mut actual = before ; result . apply (& mut actual) ; actual } ; assert_eq_text ! (ra_fixture_after , & actual) ; } # [test] fn test_join_lines_comma () { check_join_lines (r"
fn foo() {
    $0foo(1,
    )
}
" , r"
fn foo() {
    $0foo(1)
}
" ,) ; } # [test] fn test_join_lines_lambda_block () { check_join_lines (r"
pub fn reparse(&self, edit: &AtomTextEdit) -> File {
    $0self.incremental_reparse(edit).unwrap_or_else(|| {
        self.full_reparse(edit)
    })
}
" , r"
pub fn reparse(&self, edit: &AtomTextEdit) -> File {
    $0self.incremental_reparse(edit).unwrap_or_else(|| self.full_reparse(edit))
}
" ,) ; } # [test] fn test_join_lines_block () { check_join_lines (r"
fn foo() {
    foo($0{
        92
    })
}" , r"
fn foo() {
    foo($092)
}" ,) ; } # [test] fn test_join_lines_diverging_block () { check_join_lines (r"
fn foo() {
    loop {
        match x {
            92 => $0{
                continue;
            }
        }
    }
}
        " , r"
fn foo() {
    loop {
        match x {
            92 => $0continue,
        }
    }
}
        " ,) ; } # [test] fn join_lines_adds_comma_for_block_in_match_arm () { check_join_lines (r"
fn foo(e: Result<U, V>) {
    match e {
        Ok(u) => $0{
            u.foo()
        }
        Err(v) => v,
    }
}" , r"
fn foo(e: Result<U, V>) {
    match e {
        Ok(u) => $0u.foo(),
        Err(v) => v,
    }
}" ,) ; } # [test] fn join_lines_multiline_in_block () { check_join_lines (r"
fn foo() {
    match ty {
        $0 Some(ty) => {
            match ty {
                _ => false,
            }
        }
        _ => true,
    }
}
" , r"
fn foo() {
    match ty {
        $0 Some(ty) => match ty {
                _ => false,
            },
        _ => true,
    }
}
" ,) ; } # [test] fn join_lines_keeps_comma_for_block_in_match_arm () { check_join_lines (r"
fn foo(e: Result<U, V>) {
    match e {
        Ok(u) => $0{
            u.foo()
        },
        Err(v) => v,
    }
}" , r"
fn foo(e: Result<U, V>) {
    match e {
        Ok(u) => $0u.foo(),
        Err(v) => v,
    }
}" ,) ; check_join_lines (r"
fn foo(e: Result<U, V>) {
    match e {
        Ok(u) => $0{
            u.foo()
        }    ,
        Err(v) => v,
    }
}" , r"
fn foo(e: Result<U, V>) {
    match e {
        Ok(u) => $0u.foo()    ,
        Err(v) => v,
    }
}" ,) ; check_join_lines (r"
fn foo(e: Result<U, V>) {
    match e {
        Ok(u) => $0{
            u.foo()
        }
        ,
        Err(v) => v,
    }
}" , r"
fn foo(e: Result<U, V>) {
    match e {
        Ok(u) => $0u.foo()
        ,
        Err(v) => v,
    }
}" ,) ; } # [test] fn join_lines_keeps_comma_with_single_arg_tuple () { check_join_lines (r"
fn foo() {
    let x = ($0{
       4
    },);
}" , r"
fn foo() {
    let x = ($04,);
}" ,) ; check_join_lines (r"
fn foo() {
    let x = ($0{
       4
    }   ,);
}" , r"
fn foo() {
    let x = ($04   ,);
}" ,) ; check_join_lines (r"
fn foo() {
    let x = ($0{
       4
    }
    ,);
}" , r"
fn foo() {
    let x = ($04
    ,);
}" ,) ; } # [test] fn test_join_lines_use_items_left () { check_join_lines (r"
$0use syntax::{
    TextSize, TextRange,
};" , r"
$0use syntax::{TextSize, TextRange,
};" ,) ; } # [test] fn test_join_lines_use_items_right () { check_join_lines (r"
use syntax::{
$0    TextSize, TextRange
};" , r"
use syntax::{
$0    TextSize, TextRange};" ,) ; } # [test] fn test_join_lines_use_items_right_comma () { check_join_lines (r"
use syntax::{
$0    TextSize, TextRange,
};" , r"
use syntax::{
$0    TextSize, TextRange};" ,) ; } # [test] fn test_join_lines_use_tree () { check_join_lines (r"
use syntax::{
    algo::$0{
        find_token_at_offset,
    },
    ast,
};" , r"
use syntax::{
    algo::$0find_token_at_offset,
    ast,
};" ,) ; } # [test] fn test_join_lines_normal_comments () { check_join_lines (r"
fn foo() {
    // Hello$0
    // world!
}
" , r"
fn foo() {
    // Hello$0 world!
}
" ,) ; } # [test] fn test_join_lines_doc_comments () { check_join_lines (r"
fn foo() {
    /// Hello$0
    /// world!
}
" , r"
fn foo() {
    /// Hello$0 world!
}
" ,) ; } # [test] fn test_join_lines_mod_comments () { check_join_lines (r"
fn foo() {
    //! Hello$0
    //! world!
}
" , r"
fn foo() {
    //! Hello$0 world!
}
" ,) ; } # [test] fn test_join_lines_multiline_comments_1 () { check_join_lines (r"
fn foo() {
    // Hello$0
    /* world! */
}
" , r"
fn foo() {
    // Hello$0 world! */
}
" ,) ; } # [test] fn test_join_lines_multiline_comments_2 () { check_join_lines (r"
fn foo() {
    // The$0
    /* quick
    brown
    fox! */
}
" , r"
fn foo() {
    // The$0 quick
    brown
    fox! */
}
" ,) ; } # [test] fn test_join_lines_selection_fn_args () { check_join_lines_sel (r"
fn foo() {
    $0foo(1,
        2,
        3,
    $0)
}
    " , r"
fn foo() {
    foo(1, 2, 3)
}
    " ,) ; } # [test] fn test_join_lines_selection_struct () { check_join_lines_sel (r"
struct Foo $0{
    f: u32,
}$0
    " , r"
struct Foo { f: u32 }
    " ,) ; } # [test] fn test_join_lines_selection_dot_chain () { check_join_lines_sel (r"
fn foo() {
    join($0type_params.type_params()
            .filter_map(|it| it.name())
            .map(|it| it.text())$0)
}" , r"
fn foo() {
    join(type_params.type_params().filter_map(|it| it.name()).map(|it| it.text()))
}" ,) ; } # [test] fn test_join_lines_selection_lambda_block_body () { check_join_lines_sel (r"
pub fn handle_find_matching_brace() {
    params.offsets
        .map(|offset| $0{
            world.analysis().matching_brace(&file, offset).unwrap_or(offset)
        }$0)
        .collect();
}" , r"
pub fn handle_find_matching_brace() {
    params.offsets
        .map(|offset| world.analysis().matching_brace(&file, offset).unwrap_or(offset))
        .collect();
}" ,) ; } # [test] fn test_join_lines_commented_block () { check_join_lines (r"
fn main() {
    let _ = {
        // $0foo
        // bar
        92
    };
}
        " , r"
fn main() {
    let _ = {
        // $0foo bar
        92
    };
}
        " ,) } # [test] fn join_lines_mandatory_blocks_block () { check_join_lines (r"
$0fn foo() {
    92
}
        " , r"
$0fn foo() { 92
}
        " ,) ; check_join_lines (r"
fn foo() {
    $0if true {
        92
    }
}
        " , r"
fn foo() {
    $0if true { 92
    }
}
        " ,) ; check_join_lines (r"
fn foo() {
    $0loop {
        92
    }
}
        " , r"
fn foo() {
    $0loop { 92
    }
}
        " ,) ; check_join_lines (r"
fn foo() {
    $0unsafe {
        92
    }
}
        " , r"
fn foo() {
    $0unsafe { 92
    }
}
        " ,) ; } # [test] fn join_string_literal () { { cov_mark :: check ! (join_string_literal_open_quote) ; check_join_lines (r#"
fn main() {
    $0"
hello
";
}
"# , r#"
fn main() {
    $0"hello
";
}
"# ,) ; } { cov_mark :: check ! (join_string_literal_close_quote) ; check_join_lines (r#"
fn main() {
    $0"hello
";
}
"# , r#"
fn main() {
    $0"hello";
}
"# ,) ; check_join_lines (r#"
fn main() {
    $0r"hello
    ";
}
"# , r#"
fn main() {
    $0r"hello";
}
"# ,) ; } check_join_lines (r#"
fn main() {
    "
$0hello
world
";
}
"# , r#"
fn main() {
    "
$0hello world
";
}
"# ,) ; } # [test] fn join_last_line_empty () { check_join_lines (r#"
fn main() {$0}
"# , r#"
fn main() {$0}
"# ,) ; } # [test] fn join_two_ifs () { cov_mark :: check ! (join_two_ifs) ; check_join_lines (r#"
fn main() {
    if foo {

    }$0
    if bar {

    }
}
"# , r#"
fn main() {
    if foo {

    }$0 else if bar {

    }
}
"# ,) ; } # [test] fn join_two_ifs_with_existing_else () { cov_mark :: check ! (join_two_ifs_with_existing_else) ; check_join_lines (r#"
fn main() {
    if foo {

    } else {

    }$0
    if bar {

    }
}
"# , r#"
fn main() {
    if foo {

    } else {

    }$0 if bar {

    }
}
"# ,) ; } # [test] fn join_assignments () { check_join_lines (r#"
fn foo() {
    $0let foo;
    foo = "bar";
}
"# , r#"
fn foo() {
    $0let foo = "bar";
}
"# ,) ; cov_mark :: check ! (join_assignments_mismatch) ; check_join_lines (r#"
fn foo() {
    let foo;
    let qux;$0
    foo = "bar";
}
"# , r#"
fn foo() {
    let foo;
    let qux;$0 foo = "bar";
}
"# ,) ; cov_mark :: check ! (join_assignments_already_initialized) ; check_join_lines (r#"
fn foo() {
    let foo = "bar";$0
    foo = "bar";
}
"# , r#"
fn foo() {
    let foo = "bar";$0 foo = "bar";
}
"# ,) ; } }
};
}
