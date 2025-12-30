// Generated macro for tests (module)
macro_rules! Depcrate_matching_bracetests {
() => {
// Module: crate::matching_brace
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use test_utils :: { add_cursor , assert_eq_text , extract_offset } ; use super :: * ; # [test] fn test_matching_brace () { fn do_check (before : & str , after : & str) { let (pos , before) = extract_offset (before) ; let parse = SourceFile :: parse (& before , span :: Edition :: CURRENT) ; let new_pos = match matching_brace (& parse . tree () , pos) { None => pos , Some (pos) => pos , } ; let actual = add_cursor (& before , new_pos) ; assert_eq_text ! (after , & actual) ; } do_check ("struct Foo { a: i32, }$0" , "struct Foo $0{ a: i32, }") ; do_check ("fn main() { |x: i32|$0 x * 2;}" , "fn main() { $0|x: i32| x * 2;}") ; do_check ("fn main() { $0|x: i32| x * 2;}" , "fn main() { |x: i32$0| x * 2;}") ; do_check ("fn func(x) { return (2 * (x + 3)$0) + 5;}" , "fn func(x) { return $0(2 * (x + 3)) + 5;}" ,) ; { cov_mark :: check ! (pipes_not_braces) ; do_check ("fn main() { match 92 { 1 | 2 |$0 3 => 92 } }" , "fn main() { match 92 { 1 | 2 |$0 3 => 92 } }" ,) ; } } }
};
}
