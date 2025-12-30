// Generated macro for tests (module)
macro_rules! Depcrate_syntax_texttests {
() => {
// Module: crate::syntax_text
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: { GreenNodeBuilder , green :: SyntaxKind } ; use super :: * ; fn build_tree (chunks : & [& str]) -> SyntaxNode { let mut builder = GreenNodeBuilder :: new () ; builder . start_node (SyntaxKind (62)) ; for & chunk in chunks . iter () { builder . token (SyntaxKind (92) , chunk) } builder . finish_node () ; SyntaxNode :: new_root (builder . finish ()) } # [test] fn test_text_equality () { fn do_check (t1 : & [& str] , t2 : & [& str]) { let t1 = build_tree (t1) . text () ; let t2 = build_tree (t2) . text () ; let expected = t1 . to_string () == t2 . to_string () ; let actual = t1 == t2 ; assert_eq ! (expected , actual , "`{}` (SyntaxText) `{}` (SyntaxText)" , t1 , t2) ; let actual = t1 == * t2 . to_string () ; assert_eq ! (expected , actual , "`{}` (SyntaxText) `{}` (&str)" , t1 , t2) ; } fn check (t1 : & [& str] , t2 : & [& str]) { do_check (t1 , t2) ; do_check (t2 , t1) } check (& [""] , & [""]) ; check (& ["a"] , & [""]) ; check (& ["a"] , & ["a"]) ; check (& ["abc"] , & ["def"]) ; check (& ["hello" , "world"] , & ["hello" , "world"]) ; check (& ["hellowo" , "rld"] , & ["hell" , "oworld"]) ; check (& ["hel" , "lowo" , "rld"] , & ["helloworld"]) ; check (& ["{" , "abc" , "}"] , & ["{" , "123" , "}"]) ; check (& ["{" , "abc" , "}" , "{"] , & ["{" , "123" , "}"]) ; check (& ["{" , "abc" , "}"] , & ["{" , "123" , "}" , "{"]) ; check (& ["{" , "abc" , "}ab"] , & ["{" , "abc" , "}" , "ab"]) ; } }
};
}
