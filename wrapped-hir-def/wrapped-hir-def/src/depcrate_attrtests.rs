// Generated macro for tests (module)
macro_rules! Depcrate_attrtests {
() => {
// Module: crate::attr
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # ! [doc = " This module contains tests for doc-expression parsing."] # ! [doc = " Currently, it tests `#[doc(hidden)]` and `#[doc(alias)]`."] use intern :: Symbol ; use span :: EditionedFileId ; use triomphe :: Arc ; use hir_expand :: span_map :: { RealSpanMap , SpanMap } ; use span :: FileId ; use syntax :: { AstNode , TextRange , ast } ; use syntax_bridge :: { DocCommentDesugarMode , syntax_node_to_token_tree } ; use crate :: attr :: { DocAtom , DocExpr } ; fn assert_parse_result (input : & str , expected : DocExpr) { let source_file = ast :: SourceFile :: parse (input , span :: Edition :: CURRENT) . ok () . unwrap () ; let tt = source_file . syntax () . descendants () . find_map (ast :: TokenTree :: cast) . unwrap () ; let map = SpanMap :: RealSpanMap (Arc :: new (RealSpanMap :: absolute (EditionedFileId :: current_edition (FileId :: from_raw (0)) ,))) ; let tt = syntax_node_to_token_tree (tt . syntax () , map . as_ref () , map . span_for_range (TextRange :: empty (0 . into ())) , DocCommentDesugarMode :: ProcMacro ,) ; let cfg = DocExpr :: parse (& tt) ; assert_eq ! (cfg , expected) ; } # [test] fn test_doc_expr_parser () { assert_parse_result ("#![doc(hidden)]" , DocAtom :: Flag (Symbol :: intern ("hidden")) . into ()) ; assert_parse_result (r#"#![doc(alias = "foo")]"# , DocAtom :: KeyValue { key : Symbol :: intern ("alias") , value : Symbol :: intern ("foo") } . into () ,) ; assert_parse_result (r#"#![doc(alias("foo"))]"# , DocExpr :: Alias ([Symbol :: intern ("foo")] . into ()) ,) ; assert_parse_result (r#"#![doc(alias("foo", "bar", "baz"))]"# , DocExpr :: Alias ([Symbol :: intern ("foo") , Symbol :: intern ("bar") , Symbol :: intern ("baz")] . into () ,) ,) ; assert_parse_result (r#"
        #[doc(alias("Bar", "Qux"))]
        struct Foo;"# , DocExpr :: Alias ([Symbol :: intern ("Bar") , Symbol :: intern ("Qux")] . into ()) ,) ; } }
};
}
