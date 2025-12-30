// Generated macro for impl_102 (impl)
macro_rules! Depcrate_generatedimpl_102 {
() => {
// Module: crate::generated
// Provides: {"impl_102"}
// Dependencies: {}
impl XCTestSuite { extern_methods ! (# [unsafe (method (defaultTestSuite))] # [unsafe (method_family = none)] pub fn defaultTestSuite () -> Retained < XCTestSuite >; # [unsafe (method (testSuiteForBundlePath :))] # [unsafe (method_family = none)] pub fn testSuiteForBundlePath (bundle_path : & NSString) -> Retained < Self >; # [unsafe (method (testSuiteForTestCaseWithName :))] # [unsafe (method_family = none)] pub fn testSuiteForTestCaseWithName (name : & NSString) -> Retained < Self >; # [doc = " # Safety"] # [doc = ""] # [doc = " `test_case_class` probably has further requirements."] # [unsafe (method (testSuiteForTestCaseClass :))] # [unsafe (method_family = none)] pub unsafe fn testSuiteForTestCaseClass (test_case_class : & AnyClass) -> Retained < Self >; # [unsafe (method (testSuiteWithName :))] # [unsafe (method_family = none)] pub fn testSuiteWithName (name : & NSString) -> Retained < Self >; # [unsafe (method (initWithName :))] # [unsafe (method_family = init)] pub fn initWithName (this : Allocated < Self >, name : & NSString) -> Retained < Self >; # [unsafe (method (new))] # [unsafe (method_family = new)] pub unsafe fn new () -> Retained < Self >; # [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [unsafe (method (addTest :))] # [unsafe (method_family = none)] pub fn addTest (& self , test : & XCTest) ; # [unsafe (method (tests))] # [unsafe (method_family = none)] pub fn tests (& self) -> Retained < NSArray < XCTest >>;) ; }
};
}
