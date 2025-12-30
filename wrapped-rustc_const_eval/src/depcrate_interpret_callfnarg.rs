// Generated macro for FnArg (enum)
macro_rules! Depcrate_interpret_callFnArg {
() => {
// Module: crate::interpret::call
// Provides: {"FnArg"}
// Dependencies: {}
# [doc = " An argument passed to a function."] # [derive (Clone , Debug)] pub enum FnArg < 'tcx , Prov : Provenance = CtfeProvenance > { # [doc = " Pass a copy of the given operand."] Copy (OpTy < 'tcx , Prov >) , # [doc = " Allow for the argument to be passed in-place: destroy the value originally stored at that"] # [doc = " place and make the place inaccessible for the duration of the function call. This *must* be"] # [doc = " an in-memory place so that we can do the proper alias checks."] InPlace (MPlaceTy < 'tcx , Prov >) , }
};
}
