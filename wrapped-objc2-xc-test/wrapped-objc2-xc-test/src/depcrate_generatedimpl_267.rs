// Generated macro for impl_267 (impl)
macro_rules! Depcrate_generatedimpl_267 {
() => {
// Module: crate::generated
// Provides: {"impl_267"}
// Dependencies: {}
impl XCTSourceCodeContext { extern_methods ! (# [unsafe (method (initWithCallStack : location :))] # [unsafe (method_family = init)] pub fn initWithCallStack_location (this : Allocated < Self >, call_stack : & NSArray < XCTSourceCodeFrame >, location : Option <& XCTSourceCodeLocation >,) -> Retained < Self >; # [doc = " The call stack addresses could be those from NSThread.callStackReturnAddresses,"] # [doc = " NSException.callStackReturnAddresses, or another source."] # [unsafe (method (initWithCallStackAddresses : location :))] # [unsafe (method_family = init)] pub fn initWithCallStackAddresses_location (this : Allocated < Self >, call_stack_addresses : & NSArray < NSNumber >, location : Option <& XCTSourceCodeLocation >,) -> Retained < Self >; # [doc = " Initializes a new instance with call stack derived from NSThread.callStackReturnAddresses and the specified location."] # [unsafe (method (initWithLocation :))] # [unsafe (method_family = init)] pub fn initWithLocation (this : Allocated < Self >, location : Option <& XCTSourceCodeLocation >,) -> Retained < Self >; # [doc = " Initializes a new instance with call stack derived from NSThread.callStackReturnAddresses and a nil location."] # [unsafe (method (init))] # [unsafe (method_family = init)] pub fn init (this : Allocated < Self >) -> Retained < Self >; # [unsafe (method (callStack))] # [unsafe (method_family = none)] pub fn callStack (& self) -> Retained < NSArray < XCTSourceCodeFrame >>; # [unsafe (method (location))] # [unsafe (method_family = none)] pub fn location (& self) -> Option < Retained < XCTSourceCodeLocation >>;) ; }
};
}
