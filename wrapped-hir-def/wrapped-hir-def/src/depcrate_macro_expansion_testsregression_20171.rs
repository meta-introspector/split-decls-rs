// Generated macro for regression_20171 (function)
macro_rules! Depcrate_macro_expansion_testsregression_20171 {
() => {
// Module: crate::macro_expansion_tests
// Provides: {"regression_20171"}
// Dependencies: {}
# [test] fn regression_20171 () { let span = Span { range : TextRange :: empty (TextSize :: new (0)) , anchor : SpanAnchor { file_id : span :: EditionedFileId :: current_edition (span :: FileId :: from_raw (0)) , ast_id : ROOT_ERASED_FILE_AST_ID , } , ctx : SyntaxContext :: root (Edition :: CURRENT) , } ; let close_brace = tt :: Punct { char : '}' , spacing : tt :: Spacing :: Alone , span } ; let dotdot1 = tt :: Punct { char : '.' , spacing : tt :: Spacing :: Joint , span } ; let dotdot2 = tt :: Punct { char : '.' , spacing : tt :: Spacing :: Alone , span } ; let dollar_crate = sym :: dollar_crate ; let tt = quote ! { span => { if ! ((matches ! (drive_parser (& mut parser , data , false) , Err (TarParserError :: CorruptField { field : CorruptFieldContext :: PaxKvLength , error : GeneralParseError :: ParseInt (ParseIntError { # dotdot1 # dotdot2 }) }) # close_brace))) { # dollar_crate :: panic :: panic_2021 ! () ; } } } ; token_tree_to_syntax_node (& tt , syntax_bridge :: TopEntryPoint :: MacroStmts , & mut | _ | Edition :: CURRENT , Edition :: CURRENT ,) ; }
};
}
