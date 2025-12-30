// Generated macro for AndThenService (struct)
macro_rules! Depcrate_and_thenAndThenService {
() => {
// Module: crate::and_then
// Provides: {"AndThenService"}
// Dependencies: {}
# [doc = " Service for the `and_then` combinator, chaining a computation onto the end of another service"] # [doc = " which completes successfully."] # [doc = ""] # [doc = " This is created by the `Pipeline::and_then` method."] pub struct AndThenService < A , B , Req > (Rc < (A , B) > , PhantomData < Req >) ;
};
}
