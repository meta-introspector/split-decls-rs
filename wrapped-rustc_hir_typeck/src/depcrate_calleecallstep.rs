// Generated macro for CallStep (enum)
macro_rules! Depcrate_calleeCallStep {
() => {
// Module: crate::callee
// Provides: {"CallStep"}
// Dependencies: {}
# [derive (Debug)] enum CallStep < 'tcx > { Builtin (Ty < 'tcx >) , DeferredClosure (LocalDefId , ty :: FnSig < 'tcx >) , # [doc = " Call overloading when callee implements one of the Fn* traits."] Overloaded (MethodCallee < 'tcx >) , }
};
}
