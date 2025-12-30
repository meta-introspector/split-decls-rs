// Generated macro for impl_198 (impl)
macro_rules! Depcrate_generatedimpl_198 {
() => {
// Module: crate::generated
// Provides: {"impl_198"}
// Dependencies: {}
impl NEFilterBrowserFlow { extern_methods ! (# [doc = " The NSURLRequest of the flow. This property is always nil for the control providers."] # [unsafe (method (request))] # [unsafe (method_family = none)] pub unsafe fn request (& self) -> Option < Retained < NSURLRequest >>; # [doc = " The NSURLResponse of the flow. This will be nil until the request is sent to the server and the response headers are received. And this property is always nil for the control providers."] # [unsafe (method (response))] # [unsafe (method_family = none)] pub unsafe fn response (& self) -> Option < Retained < NSURLResponse >>; # [doc = " The parent URL for the current flow which is created to load the sub frames because the flow with the parent URL was allowed. Will be nil if the parent flow does not exist."] # [unsafe (method (parentURL))] # [unsafe (method_family = none)] pub unsafe fn parentURL (& self) -> Option < Retained < NSURL >>;) ; }
};
}
