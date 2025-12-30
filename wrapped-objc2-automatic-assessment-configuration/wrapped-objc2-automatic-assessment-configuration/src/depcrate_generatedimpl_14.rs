// Generated macro for impl_14 (impl)
macro_rules! Depcrate_generatedimpl_14 {
() => {
// Module: crate::generated
// Provides: {"impl_14"}
// Dependencies: {}
impl AEAssessmentApplication { extern_methods ! (# [unsafe (method (bundleIdentifier))] # [unsafe (method_family = none)] pub unsafe fn bundleIdentifier (& self) -> Retained < NSString >; # [unsafe (method (teamIdentifier))] # [unsafe (method_family = none)] pub unsafe fn teamIdentifier (& self) -> Option < Retained < NSString >>; # [unsafe (method (requiresSignatureValidation))] # [unsafe (method_family = none)] pub unsafe fn requiresSignatureValidation (& self) -> bool ; # [doc = " Setter for [`requiresSignatureValidation`][Self::requiresSignatureValidation]."] # [unsafe (method (setRequiresSignatureValidation :))] # [unsafe (method_family = none)] pub unsafe fn setRequiresSignatureValidation (& self , requires_signature_validation : bool) ; # [unsafe (method (initWithBundleIdentifier :))] # [unsafe (method_family = init)] pub unsafe fn initWithBundleIdentifier (this : Allocated < Self >, bundle_identifier : & NSString ,) -> Retained < Self >; # [unsafe (method (initWithBundleIdentifier : teamIdentifier :))] # [unsafe (method_family = init)] pub unsafe fn initWithBundleIdentifier_teamIdentifier (this : Allocated < Self >, bundle_identifier : & NSString , team_identifier : Option <& NSString >,) -> Retained < Self >; # [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [unsafe (method (new))] # [unsafe (method_family = new)] pub unsafe fn new () -> Retained < Self >;) ; }
};
}
