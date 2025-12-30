// Generated macro for macro_316 (macro)
macro_rules! Depcrate_fut_future_thenmacro_316 {
() => {
// Module: crate::fut::future::then
// Provides: {"macro_316"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the [`then`](super::ActorFutureExt::then) combinator, chaining computations on the end of"] # [doc = " another future regardless of its outcome."] # [doc = ""] # [doc = " This is created by the `ActorFuture::then` method."] # [project = ThenProj] # [derive (Debug)] # [must_use = "futures do nothing unless polled"] pub enum Then < A , B , F > { First { # [pin] fut1 : A , data : Option < F >, } , Second { # [pin] fut2 : B } , Empty , } }
};
}
