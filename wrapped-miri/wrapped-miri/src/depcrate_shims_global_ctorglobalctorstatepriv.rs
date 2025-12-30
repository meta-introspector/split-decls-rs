// Generated macro for GlobalCtorStatePriv (enum)
macro_rules! Depcrate_shims_global_ctorGlobalCtorStatePriv {
() => {
// Module: crate::shims::global_ctor
// Provides: {"GlobalCtorStatePriv"}
// Dependencies: {}
# [derive (Debug , Default)] enum GlobalCtorStatePriv < 'tcx > { # [default] Init , # [doc = " The list of constructor functions that we still have to call."] Ctors (Vec < ImmTy < 'tcx > >) , Done , }
};
}
