// Generated macro for OrElse (struct)
macro_rules! Depcrate_or_elseOrElse {
() => {
// Module: crate::or_else
// Provides: {"OrElse"}
// Dependencies: {}
# [doc = " Future for the `or_else` combinator, chaining a computation onto the end of"] # [doc = " a future which fails with an error."] # [doc = ""] # [doc = " This is created by this `Future::or_else` method."] pub struct OrElse < A , B , F > where A : Future , B : IntoFuture { state : Chain < A , B :: Future , F > , }
};
}
