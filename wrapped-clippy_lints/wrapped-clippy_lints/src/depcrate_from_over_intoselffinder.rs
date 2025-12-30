// Generated macro for SelfFinder (struct)
macro_rules! Depcrate_from_over_intoSelfFinder {
() => {
// Module: crate::from_over_into
// Provides: {"SelfFinder"}
// Dependencies: {}
# [doc = " Finds the occurrences of `Self` and `self`"] # [doc = ""] # [doc = " Returns `ControlFlow::break` if any of the `self`/`Self` usages were from an expansion, or the"] # [doc = " body contained a binding already named `val`."] struct SelfFinder < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , # [doc = " Occurrences of `Self`"] upper : Vec < Span > , # [doc = " Occurrences of `self`"] lower : Vec < Span > , }
};
}
