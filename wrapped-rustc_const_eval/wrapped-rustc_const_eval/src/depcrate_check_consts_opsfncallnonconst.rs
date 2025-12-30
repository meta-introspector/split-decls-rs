// Generated macro for FnCallNonConst (struct)
macro_rules! Depcrate_check_consts_opsFnCallNonConst {
() => {
// Module: crate::check_consts::ops
// Provides: {"FnCallNonConst"}
// Dependencies: {}
# [doc = " A function call where the callee is not marked as `const`."] # [derive (Debug , Clone , Copy)] pub (crate) struct FnCallNonConst < 'tcx > { pub callee : DefId , pub args : GenericArgsRef < 'tcx > , pub span : Span , pub call_source : CallSource , }
};
}
