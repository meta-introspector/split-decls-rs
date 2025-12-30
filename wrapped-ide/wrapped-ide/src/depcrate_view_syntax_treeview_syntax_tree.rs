// Generated macro for view_syntax_tree (function)
macro_rules! Depcrate_view_syntax_treeview_syntax_tree {
() => {
// Module: crate::view_syntax_tree
// Provides: {"view_syntax_tree"}
// Dependencies: {}
pub (crate) fn view_syntax_tree (db : & RootDatabase , file_id : FileId) -> String { let sema = Semantics :: new (db) ; let line_index = db . line_index (file_id) ; let parse = sema . parse_guess_edition (file_id) ; let ctx = SyntaxTreeCtx { line_index , in_string : None } ; syntax_node_to_json (parse . syntax () , & ctx) }
};
}
