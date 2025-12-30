// Generated macro for lazy_expand (function)
macro_rules! Depcrate_eagerlazy_expand {
() => {
// Module: crate::eager
// Provides: {"lazy_expand"}
// Dependencies: {}
fn lazy_expand (db : & dyn ExpandDatabase , def : & MacroDefId , macro_call : & ast :: MacroCall , ast_id : AstId < ast :: MacroCall > , krate : Crate , call_site : SyntaxContext , eager_callback : EagerCallBackFn < '_ > ,) -> ExpandResult < (InFile < Parse < SyntaxNode > > , Arc < ExpansionSpanMap >) > { let expand_to = ExpandTo :: from_call_site (macro_call) ; let id = def . make_call (db , krate , MacroCallKind :: FnLike { ast_id , expand_to , eager : None } , call_site ,) ; eager_callback (ast_id . map (| ast_id | (AstPtr :: new (macro_call) , ast_id)) , id) ; db . parse_macro_expansion (id) . map (| parse | (InFile :: new (id . into () , parse . 0) , parse . 1)) }
};
}
