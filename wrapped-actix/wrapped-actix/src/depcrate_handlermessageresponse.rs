// Generated macro for MessageResponse (trait)
macro_rules! Depcrate_handlerMessageResponse {
() => {
// Module: crate::handler
// Provides: {"MessageResponse"}
// Dependencies: {}
# [doc = " A trait which defines message responses."] # [doc = ""] # [doc = " We offer implementation for some common language types, if you need"] # [doc = " to respond with a new type you can use [`MessageResult`]."] # [doc = ""] # [doc = " If `Actor::Context` implements [`AsyncContext`] it's possible to handle"] # [doc = " the message asynchronously."] # [doc = " For asynchronous message handling we offer the following possible response types:"] # [doc = " - [`ResponseFuture`] should be used for when the future returned doesn't"] # [doc = "   need to access Actor's internal state or context to progress, either"] # [doc = "   because it's completely agnostic to it or because the required data has"] # [doc = "   already been moved to it and it won't need Actor state to continue."] # [doc = " - [`ResponseActFuture`] should be used when the future returned"] # [doc = "   will, at some point, need to access Actor's internal state or context"] # [doc = "   in order to finish."] # [doc = " - [`AtomicResponse`] should be used when the future returned needs exclusive"] # [doc = "   access to  Actor's internal state or context."] pub trait MessageResponse < A : Actor , M : Message > { fn handle (self , ctx : & mut A :: Context , tx : Option < OneshotSender < M :: Result > >) ; }
};
}
