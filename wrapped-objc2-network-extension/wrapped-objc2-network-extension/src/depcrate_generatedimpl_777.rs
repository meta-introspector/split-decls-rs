// Generated macro for impl_777 (impl)
macro_rules! Depcrate_generatedimpl_777 {
() => {
// Module: crate::generated
// Provides: {"impl_777"}
// Dependencies: {}
impl NEURLFilter { extern_methods ! (# [cfg (feature = "block2")] # [doc = " This method determines if the specified URL should be allowed or denied.  The returned Allow or Deny verdict should be honored to prevent"] # [doc = " communication with restricted or malicious Internet sites."] # [doc = " - Parameters:"] # [doc = " - url: url to be validated"] # [doc = " - completionHandler: A block that will be called when validation is completed. A NEURLFilterVerdict verdict will be returned to indicate"] # [doc = " whether the specified URL should be allowed or denied.  If verdict is Deny, caller should fail the URL request."] # [unsafe (method (verdictForURL : completionHandler :))] # [unsafe (method_family = none)] pub unsafe fn verdictForURL_completionHandler (url : & NSURL , completion_handler : & block2 :: DynBlock < dyn Fn (NEURLFilterVerdict) >,) ; # [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >;) ; }
};
}
