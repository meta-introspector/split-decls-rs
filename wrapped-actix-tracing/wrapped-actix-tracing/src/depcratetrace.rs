// Generated macro for trace (function)
macro_rules! Depcratetrace {
() => {
// Module: crate
// Provides: {"trace"}
// Dependencies: {}
# [doc = " Wraps the provided service factory with a transform that automatically"] # [doc = " enters/exits the given span."] # [doc = ""] # [doc = " The span to be entered/exited can be provided via a closure. The closure"] # [doc = " is passed in a reference to the request being handled by the service."] # [doc = ""] # [doc = " For example:"] # [doc = " ```ignore"] # [doc = " let traced_service = trace("] # [doc = "     web_service,"] # [doc = "     |req: &Request| Some(span!(Level::INFO, \"request\", req.id))"] # [doc = " );"] # [doc = " ```"] pub fn trace < S , Req , I , F > (service_factory : I , make_span : F ,) -> ApplyTransform < TracingTransform < S :: Service , S , F > , S , Req > where I : IntoServiceFactory < S , Req > , S : ServiceFactory < Req > , F : Fn (& Req) -> Option < tracing :: Span > + Clone , { apply (TracingTransform :: new (make_span) , service_factory . into_factory () ,) }
};
}
