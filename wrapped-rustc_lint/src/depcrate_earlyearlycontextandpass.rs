// Generated macro for EarlyContextAndPass (struct)
macro_rules! Depcrate_earlyEarlyContextAndPass {
() => {
// Module: crate::early
// Provides: {"EarlyContextAndPass"}
// Dependencies: {}
# [doc = " Implements the AST traversal for early lint passes. `T` provides the"] # [doc = " `check_*` methods."] pub struct EarlyContextAndPass < 'ecx , 'tcx , T : EarlyLintPass > { context : EarlyContext < 'ecx > , tcx : Option < TyCtxt < 'tcx > > , pass : T , }
};
}
