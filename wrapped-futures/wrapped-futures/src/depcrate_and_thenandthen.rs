// Generated macro for AndThen (struct)
macro_rules! Depcrate_and_thenAndThen {
() => {
// Module: crate::and_then
// Provides: {"AndThen"}
// Dependencies: {}
# [doc = " Future for the `and_then` combinator, chaining a computation onto the end of"] # [doc = " another future which completes successfully."] # [doc = ""] # [doc = " This is created by this `Future::and_then` method."] pub struct AndThen < A , B , F > where A : Future , B : IntoFuture { state : Chain < A , B :: Future , F > , }
};
}
