// Generated macro for vars_since_snapshot (function)
macro_rules! Depcrate_infer_snapshot_fudgevars_since_snapshot {
() => {
// Module: crate::infer::snapshot::fudge
// Provides: {"vars_since_snapshot"}
// Dependencies: {}
fn vars_since_snapshot < 'tcx , T > (table : & UnificationTable < '_ , 'tcx , T > , snapshot_var_len : usize ,) -> Range < T > where T : UnifyKey , super :: UndoLog < 'tcx > : From < sv :: UndoLog < ut :: Delegate < T > > > , { T :: from_index (snapshot_var_len as u32) .. T :: from_index (table . len () as u32) }
};
}
