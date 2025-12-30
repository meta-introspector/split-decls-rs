// Generated macro for macro_431 (macro)
macro_rules! Depcrate_fut_try_future_and_thenmacro_431 {
() => {
// Module: crate::fut::try_future::and_then
// Provides: {"macro_431"}
// Dependencies: {}
pin_project ! { # [doc = " Future for the `and_then` combinator, chaining computations"] # [doc = " on the end of another actor future regardless of its outcome."] # [doc = ""] # [doc = " This is created by the `ActorTryFuture::and_then` method."] # [project = AndThenProj] # [derive (Debug)] # [must_use = "futures do nothing unless polled"] pub enum AndThen < A , B , F > { First { # [pin] fut1 : A , data : Option < F >, } , Second { # [pin] fut2 : B } , Empty , } }
};
}
