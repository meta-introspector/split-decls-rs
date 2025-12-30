// Generated macro for impl_215 (impl)
macro_rules! Depcrate_generatedimpl_215 {
() => {
// Module: crate::generated
// Provides: {"impl_215"}
// Dependencies: {}
impl XCTApplicationLaunchMetric { extern_methods ! (# [doc = " Initializes an application launch metric that measures the amount of time an"] # [doc = " application takes to display its first frame to screen."] # [unsafe (method (init))] # [unsafe (method_family = init)] pub fn init (this : Allocated < Self >) -> Retained < Self >; # [doc = " Initializes an application launch metric that measures the amount of time it takes"] # [doc = " for an application to display its first frame to screen and for its main thread to be"] # [doc = " ready to accept user input."] # [doc = ""] # [doc = ""] # [doc = " Parameter `waitUntilResponsive`: Specifies the end of the application launch"] # [doc = " interval to be when the application's main thread is responsive to user input."] # [unsafe (method (initWithWaitUntilResponsive :))] # [unsafe (method_family = init)] pub fn initWithWaitUntilResponsive (this : Allocated < Self >, wait_until_responsive : bool ,) -> Retained < Self >;) ; }
};
}
