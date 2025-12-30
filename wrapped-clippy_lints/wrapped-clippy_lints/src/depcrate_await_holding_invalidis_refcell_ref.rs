// Generated macro for is_refcell_ref (function)
macro_rules! Depcrate_await_holding_invalidis_refcell_ref {
() => {
// Module: crate::await_holding_invalid
// Provides: {"is_refcell_ref"}
// Dependencies: {}
fn is_refcell_ref (cx : & LateContext < '_ > , def_id : DefId) -> bool { matches ! (cx . tcx . get_diagnostic_name (def_id) , Some (sym :: RefCellRef | sym :: RefCellRefMut)) }
};
}
