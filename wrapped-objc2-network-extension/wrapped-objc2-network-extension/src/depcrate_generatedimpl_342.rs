// Generated macro for impl_342 (impl)
macro_rules! Depcrate_generatedimpl_342 {
() => {
// Module: crate::generated
// Provides: {"impl_342"}
// Dependencies: {}
impl NEFlowMetaData { extern_methods ! (# [doc = " A byte string that uniquely identifies the binary for each build of the source application of the flow. The data object may be empty in cases where the flow originates from a system process."] # [unsafe (method (sourceAppUniqueIdentifier))] # [unsafe (method_family = none)] pub unsafe fn sourceAppUniqueIdentifier (& self) -> Retained < NSData >; # [doc = " A string containing the signing identifier (almost always equivalent to the bundle identifier) of the source app of the flow. The string may be empty in cases where the flow originates from a system process."] # [unsafe (method (sourceAppSigningIdentifier))] # [unsafe (method_family = none)] pub unsafe fn sourceAppSigningIdentifier (& self) -> Retained < NSString >; # [doc = " Audit token of the source application of the flow."] # [unsafe (method (sourceAppAuditToken))] # [unsafe (method_family = none)] pub unsafe fn sourceAppAuditToken (& self) -> Option < Retained < NSData >>; # [doc = " The identifier of the content filter flow corresponding to this flow."] # [unsafe (method (filterFlowIdentifier))] # [unsafe (method_family = none)] pub unsafe fn filterFlowIdentifier (& self) -> Option < Retained < NSUUID >>;) ; }
};
}
