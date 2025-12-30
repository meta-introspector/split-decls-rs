// Generated macro for impl_136 (impl)
macro_rules! Depcrate_generatedimpl_136 {
() => {
// Module: crate::generated
// Provides: {"impl_136"}
// Dependencies: {}
impl XCUILocation { extern_methods ! (# [unsafe (method (new))] # [unsafe (method_family = new)] pub fn new (mtm : MainThreadMarker) -> Retained < Self >; # [unsafe (method (init))] # [unsafe (method_family = init)] pub fn init (this : Allocated < Self >) -> Retained < Self >; # [cfg (feature = "objc2-core-location")] # [unsafe (method (initWithLocation :))] # [unsafe (method_family = init)] pub fn initWithLocation (this : Allocated < Self >, location : & CLLocation) -> Retained < Self >; # [doc = " Provides debugging information about the underlying CLLocation wrapped by this object."] # [unsafe (method (debugDescription))] # [unsafe (method_family = none)] pub fn debugDescription (& self) -> Retained < NSString >; # [cfg (feature = "objc2-core-location")] # [doc = " Provides access to the CLLocation object stored by this XCUILocation instance."] # [unsafe (method (location))] # [unsafe (method_family = none)] pub fn location (& self) -> Retained < CLLocation >;) ; }
};
}
