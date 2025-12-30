// Generated macro for mir_assign_valid_types (function)
macro_rules! Depcrate_interpret_eval_contextmir_assign_valid_types {
() => {
// Module: crate::interpret::eval_context
// Provides: {"mir_assign_valid_types"}
// Dependencies: {}
# [doc = " Test if it is valid for a MIR assignment to assign `src`-typed place to `dest`-typed value."] # [doc = " This test should be symmetric, as it is primarily about layout compatibility."] pub (super) fn mir_assign_valid_types < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx > , src : TyAndLayout < 'tcx > , dest : TyAndLayout < 'tcx > ,) -> bool { if util :: relate_types (tcx , typing_env , Variance :: Covariant , src . ty , dest . ty) { if cfg ! (debug_assertions) || src . ty != dest . ty { assert_eq ! (src . layout , dest . layout) ; } true } else { false } }
};
}
