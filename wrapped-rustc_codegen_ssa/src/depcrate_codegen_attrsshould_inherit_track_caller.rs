// Generated macro for should_inherit_track_caller (function)
macro_rules! Depcrate_codegen_attrsshould_inherit_track_caller {
() => {
// Module: crate::codegen_attrs
// Provides: {"should_inherit_track_caller"}
// Dependencies: {}
# [doc = " Checks if the provided DefId is a method in a trait impl for a trait which has track_caller"] # [doc = " applied to the method prototype."] fn should_inherit_track_caller (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { tcx . trait_item_of (def_id) . is_some_and (| id | { tcx . codegen_fn_attrs (id) . flags . intersects (CodegenFnAttrFlags :: TRACK_CALLER) }) }
};
}
