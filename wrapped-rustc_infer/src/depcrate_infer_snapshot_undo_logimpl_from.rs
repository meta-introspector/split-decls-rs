// Generated macro for impl_from (macro)
macro_rules! Depcrate_infer_snapshot_undo_logimpl_from {
() => {
// Module: crate::infer::snapshot::undo_log
// Provides: {"impl_from"}
// Dependencies: {}
macro_rules ! impl_from { ($ ($ ctor : ident ($ ty : ty) ,) *) => { $ (impl <'tcx > From <$ ty > for UndoLog <'tcx > { fn from (x : $ ty) -> Self { UndoLog ::$ ctor (x . into ()) } }) * } }
};
}
