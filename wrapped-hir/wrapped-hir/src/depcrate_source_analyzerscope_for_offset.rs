// Generated macro for scope_for_offset (function)
macro_rules! Depcrate_source_analyzerscope_for_offset {
() => {
// Module: crate::source_analyzer
// Provides: {"scope_for_offset"}
// Dependencies: {}
fn scope_for_offset (db : & dyn HirDatabase , scopes : & ExprScopes , source_map : & BodySourceMap , from_file : HirFileId , offset : TextSize ,) -> Option < ScopeId > { scopes . scope_by_expr () . iter () . filter_map (| (id , scope) | { let InFile { file_id , value } = source_map . expr_syntax (id) . ok () ? ; if from_file == file_id { return Some ((value . text_range () , scope)) ; } let source = iter :: successors (file_id . macro_file () . map (| it | it . call_node (db)) , | it | { Some (it . file_id . macro_file () ? . call_node (db)) }) . find (| it | it . file_id == from_file) . filter (| it | it . kind () == SyntaxKind :: MACRO_CALL) ? ; Some ((source . text_range () , scope)) }) . filter (| (expr_range , _scope) | expr_range . start () <= offset && offset <= expr_range . end ()) . min_by_key (| (expr_range , _scope) | expr_range . len ()) . map (| (expr_range , scope) | { adjust (db , scopes , source_map , expr_range , from_file , offset) . unwrap_or (* scope) }) }
};
}
