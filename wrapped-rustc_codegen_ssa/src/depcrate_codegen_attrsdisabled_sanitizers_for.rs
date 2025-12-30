// Generated macro for disabled_sanitizers_for (function)
macro_rules! Depcrate_codegen_attrsdisabled_sanitizers_for {
() => {
// Module: crate::codegen_attrs
// Provides: {"disabled_sanitizers_for"}
// Dependencies: {}
fn disabled_sanitizers_for (tcx : TyCtxt < '_ > , did : LocalDefId) -> SanitizerSet { let mut disabled = match tcx . opt_local_parent (did) { Some (parent) => tcx . disabled_sanitizers_for (parent) , None => SanitizerSet :: empty () , } ; if let Some ((on_set , off_set)) = find_attr ! (tcx . get_all_attrs (did) , AttributeKind :: Sanitize { on_set , off_set , .. } => (on_set , off_set)) { disabled &= ! * on_set ; disabled |= * off_set ; } disabled }
};
}
