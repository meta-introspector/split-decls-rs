// Generated macro for id_to_string (function)
macro_rules! Depcrateid_to_string {
() => {
// Module: crate
// Provides: {"id_to_string"}
// Dependencies: {}
pub fn id_to_string (cx : & dyn rustc_hir :: intravisit :: HirTyCtxt < '_ > , hir_id : HirId) -> String { to_string (& cx , | s | s . print_node (cx . hir_node (hir_id))) }
};
}
