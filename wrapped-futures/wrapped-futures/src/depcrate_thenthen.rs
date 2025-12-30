// Generated macro for Then (struct)
macro_rules! Depcrate_thenThen {
() => {
// Module: crate::then
// Provides: {"Then"}
// Dependencies: {}
# [doc = " Future for the `then` combinator, chaining computations on the end of"] # [doc = " another future regardless of its outcome."] # [doc = ""] # [doc = " This is created by this `Future::then` method."] pub struct Then < A , B , F > where A : Future , B : IntoFuture { state : Chain < A , B :: Future , F > , }
};
}
