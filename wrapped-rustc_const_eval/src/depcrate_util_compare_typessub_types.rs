// Generated macro for sub_types (function)
macro_rules! Depcrate_util_compare_typessub_types {
() => {
// Module: crate::util::compare_types
// Provides: {"sub_types"}
// Dependencies: {}
# [doc = " Returns whether `src` is a subtype of `dest`, i.e. `src <: dest`."] pub fn sub_types < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : TypingEnv < 'tcx > , src : Ty < 'tcx > , dest : Ty < 'tcx > ,) -> bool { relate_types (tcx , typing_env , Variance :: Covariant , src , dest) }
};
}
