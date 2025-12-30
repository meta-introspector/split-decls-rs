// Generated macro for impl_284 (impl)
macro_rules! Depcrate_generatedimpl_284 {
() => {
// Module: crate::generated
// Provides: {"impl_284"}
// Dependencies: {}
impl NEFilterRemediationVerdict { extern_methods ! (# [doc = " This class method returns a verdict indicating that the flow should be allowed."] # [doc = ""] # [doc = " Returns: The NEFilterRemediationVerdict object."] # [unsafe (method (allowVerdict))] # [unsafe (method_family = none)] pub unsafe fn allowVerdict () -> Retained < NEFilterRemediationVerdict >; # [doc = " This class method returns a verdict indicating that the flow should be dropped."] # [doc = ""] # [doc = " Returns: The NEFilterRemediationVerdict object."] # [unsafe (method (dropVerdict))] # [unsafe (method_family = none)] pub unsafe fn dropVerdict () -> Retained < NEFilterRemediationVerdict >; # [doc = " This class method returns a verdict indicating that control provider needs to be asked how to handle the remediation. The control provider can either drop or allow the flow, or update the rules and ask the data provider to decide on the data flow again."] # [doc = ""] # [doc = " Returns: The NEFilterRemediationVerdict object."] # [unsafe (method (needRulesVerdict))] # [unsafe (method_family = none)] pub unsafe fn needRulesVerdict () -> Retained < NEFilterRemediationVerdict >;) ; }
};
}
