// Generated macro for impl_224 (impl)
macro_rules! Depcrate_generatedimpl_224 {
() => {
// Module: crate::generated
// Provides: {"impl_224"}
// Dependencies: {}
impl NEFilterVerdict { extern_methods ! (# [doc = " Whether or not to send a report to the control provider's -[NEFilterProvider handleReport:]"] # [doc = " method when processing this verdict and when the flow is closed. Since the data provider does not need to wait"] # [doc = " for a response from the control provider before continuing to process the flow, this is a more efficient way to"] # [doc = " report a flow to the control provider than returning a \"need rules\" verdict. If the verdict originates in the"] # [doc = " control provider, this property has no effect. This property applies when the action taken upon a flow is allow,"] # [doc = " deny, remediate, or filterData (filterData for new flows only). Setting this flag on a verdict for a socket"] # [doc = " flow will also cause the data provider's -[NEFilterProvider handleReport:] method to be called when the flow"] # [doc = " is closed."] # [unsafe (method (shouldReport))] # [unsafe (method_family = none)] pub unsafe fn shouldReport (& self) -> bool ; # [doc = " Setter for [`shouldReport`][Self::shouldReport]."] # [unsafe (method (setShouldReport :))] # [unsafe (method_family = none)] pub unsafe fn setShouldReport (& self , should_report : bool) ;) ; }
};
}
