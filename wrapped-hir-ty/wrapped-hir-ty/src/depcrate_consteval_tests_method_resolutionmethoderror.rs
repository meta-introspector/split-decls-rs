// Generated macro for MethodError (enum)
macro_rules! Depcrate_consteval_tests_method_resolutionMethodError {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"MethodError"}
// Dependencies: {}
# [derive (Debug)] pub enum MethodError < 'db > { # [doc = " Did not find an applicable method."] NoMatch , # [doc = " Multiple methods might apply."] Ambiguity (Vec < CandidateSource >) , # [doc = " Found an applicable method, but it is not visible."] PrivateMatch (Pick < 'db >) , # [doc = " Found a `Self: Sized` bound where `Self` is a trait object."] IllegalSizedBound { candidates : Vec < FunctionId > , needs_mut : bool } , # [doc = " Error has already been emitted, no need to emit another one."] ErrorReported , }
};
}
