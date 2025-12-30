// Generated macro for inherits_cfg (function)
macro_rules! Depcrateinherits_cfg {
() => {
// Module: crate
// Provides: {"inherits_cfg"}
// Dependencies: {}
# [doc = " Checks if the item of any of its parents has `#[cfg(...)]` attribute applied."] pub fn inherits_cfg (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> bool { tcx . has_attr (def_id , sym :: cfg_trace) || tcx . hir_parent_iter (tcx . local_def_id_to_hir_id (def_id)) . flat_map (| (parent_id , _) | tcx . hir_attrs (parent_id)) . any (| attr | attr . has_name (sym :: cfg_trace)) }
};
}
