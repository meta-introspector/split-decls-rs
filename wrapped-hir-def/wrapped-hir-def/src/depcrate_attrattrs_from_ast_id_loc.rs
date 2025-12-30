// Generated macro for attrs_from_ast_id_loc (function)
macro_rules! Depcrate_attrattrs_from_ast_id_loc {
() => {
// Module: crate::attr
// Provides: {"attrs_from_ast_id_loc"}
// Dependencies: {}
fn attrs_from_ast_id_loc < 'db , N : AstIdNode + HasAttrs > (db : & (dyn DefDatabase + 'db) , lookup : impl Lookup < Database = dyn DefDatabase , Data = impl AstIdLoc < Ast = N > + HasModule > ,) -> Attrs { let loc = lookup . lookup (db) ; let source = loc . source (db) ; let span_map = db . span_map (source . file_id) ; let cfg_options = loc . krate (db) . cfg_options (db) ; Attrs (RawAttrs :: new_expanded (db , & source . value , span_map . as_ref () , cfg_options)) }
};
}
