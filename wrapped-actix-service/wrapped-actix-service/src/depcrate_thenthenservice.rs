// Generated macro for ThenService (struct)
macro_rules! Depcrate_thenThenService {
() => {
// Module: crate::then
// Provides: {"ThenService"}
// Dependencies: {}
# [doc = " Service for the `then` combinator, chaining a computation onto the end of"] # [doc = " another service."] # [doc = ""] # [doc = " This is created by the `Pipeline::then` method."] pub (crate) struct ThenService < A , B , Req > (Rc < (A , B) > , PhantomData < Req >) ;
};
}
