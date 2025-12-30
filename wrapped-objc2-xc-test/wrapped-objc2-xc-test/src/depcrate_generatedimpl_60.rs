// Generated macro for impl_60 (impl)
macro_rules! Depcrate_generatedimpl_60 {
() => {
// Module: crate::generated
// Provides: {"impl_60"}
// Dependencies: {}
# [doc = " Methods declared on superclass `XCTestRun`."] impl XCTestCaseRun { extern_methods ! (# [unsafe (method (new))] # [unsafe (method_family = new)] pub unsafe fn new () -> Retained < Self >; # [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [doc = " Class factory method for the XCTestRun class."] # [doc = ""] # [doc = ""] # [doc = " Parameter `test`: An XCTest instance."] # [doc = ""] # [doc = ""] # [doc = " Returns: A test run for the provided test."] # [unsafe (method (testRunWithTest :))] # [unsafe (method_family = none)] pub fn testRunWithTest (test : & XCTest) -> Retained < Self >; # [doc = " Designated initializer for the XCTestRun class."] # [doc = ""] # [doc = ""] # [doc = " Parameter `test`: An XCTest instance."] # [doc = ""] # [doc = ""] # [doc = " Returns: A test run for the provided test."] # [unsafe (method (initWithTest :))] # [unsafe (method_family = init)] pub fn initWithTest (this : Allocated < Self >, test : & XCTest) -> Retained < Self >;) ; }
};
}
