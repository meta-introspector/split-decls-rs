// Generated macro for MiriInterpCxExt (trait)
macro_rules! Depcrate_machineMiriInterpCxExt {
() => {
// Module: crate::machine
// Provides: {"MiriInterpCxExt"}
// Dependencies: {}
# [doc = " A little trait that's useful to be inherited by extension traits."] pub trait MiriInterpCxExt < 'tcx > { fn eval_context_ref < 'a > (& 'a self) -> & 'a MiriInterpCx < 'tcx > ; fn eval_context_mut < 'a > (& 'a mut self) -> & 'a mut MiriInterpCx < 'tcx > ; }
};
}
