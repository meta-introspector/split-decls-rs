// Generated macro for impl_256 (impl)
macro_rules! Depcrate_generatedimpl_256 {
() => {
// Module: crate::generated
// Provides: {"impl_256"}
// Dependencies: {}
impl NEFilterReport { extern_methods ! (# [doc = " The flow on which the described action was taken."] # [unsafe (method (flow))] # [unsafe (method_family = none)] pub unsafe fn flow (& self) -> Option < Retained < NEFilterFlow >>; # [doc = " The action taken upon the reported flow."] # [unsafe (method (action))] # [unsafe (method_family = none)] pub unsafe fn action (& self) -> NEFilterAction ; # [doc = " The type of event that the report is reporting."] # [unsafe (method (event))] # [unsafe (method_family = none)] pub unsafe fn event (& self) -> NEFilterReportEvent ; # [doc = " The number of inbound bytes received from the flow. This property is only non-zero when the report event is NEFilterReportEventFlowClosed or NEFilterReportEventFlowStatistics."] # [unsafe (method (bytesInboundCount))] # [unsafe (method_family = none)] pub unsafe fn bytesInboundCount (& self) -> NSUInteger ; # [doc = " The number of outbound bytes sent on the flow. This property is only non-zero when the report event is NEFilterReportEventFlowClosed or NEFilterReportEventFlowStatistics."] # [unsafe (method (bytesOutboundCount))] # [unsafe (method_family = none)] pub unsafe fn bytesOutboundCount (& self) -> NSUInteger ;) ; }
};
}
