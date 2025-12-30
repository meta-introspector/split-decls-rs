// Generated macro for LateContextAndPass (struct)
macro_rules! Depcrate_lateLateContextAndPass {
() => {
// Module: crate::late
// Provides: {"LateContextAndPass"}
// Dependencies: {}
# [doc = " Implements the AST traversal for late lint passes. `T` provides the"] # [doc = " `check_*` methods."] struct LateContextAndPass < 'tcx , T : LateLintPass < 'tcx > > { context : LateContext < 'tcx > , pass : T , }
};
}
