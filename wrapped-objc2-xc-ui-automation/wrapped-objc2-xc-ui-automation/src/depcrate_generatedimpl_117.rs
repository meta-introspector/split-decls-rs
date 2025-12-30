// Generated macro for impl_117 (impl)
macro_rules! Depcrate_generatedimpl_117 {
() => {
// Module: crate::generated
// Provides: {"impl_117"}
// Dependencies: {}
impl XCUIAccessibilityAuditIssue { extern_methods ! (# [doc = " The element associated with the issue."] # [unsafe (method (element))] # [unsafe (method_family = none)] pub fn element (& self , mtm : MainThreadMarker) -> Option < Retained < XCUIElement >>; # [doc = " A short description about the issue."] # [unsafe (method (compactDescription))] # [unsafe (method_family = none)] pub fn compactDescription (& self) -> Retained < NSString >; # [doc = " A longer description of the issue with more details about the failure."] # [unsafe (method (detailedDescription))] # [unsafe (method_family = none)] pub fn detailedDescription (& self) -> Retained < NSString >; # [doc = " The type of audit which generated the issue."] # [unsafe (method (auditType))] # [unsafe (method_family = none)] pub fn auditType (& self) -> XCUIAccessibilityAuditType ; # [unsafe (method (new))] # [unsafe (method_family = new)] pub unsafe fn new () -> Retained < Self >; # [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >;) ; }
};
}
