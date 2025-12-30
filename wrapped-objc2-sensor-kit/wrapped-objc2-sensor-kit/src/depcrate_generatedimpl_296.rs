// Generated macro for impl_296 (impl)
macro_rules! Depcrate_generatedimpl_296 {
() => {
// Module: crate::generated
// Provides: {"impl_296"}
// Dependencies: {}
impl SRFaceMetricsExpression { extern_methods ! (# [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [unsafe (method (new))] # [unsafe (method_family = new)] pub unsafe fn new () -> Retained < Self >; # [doc = " An opaque identifier for the face expression"] # [doc = ""] # [doc = " More information about what this face expression represents can be found in Apple's developer documentation"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (identifier))] # [unsafe (method_family = none)] pub unsafe fn identifier (& self) -> Retained < NSString >; # [doc = " double value indicating the current position of the expression"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (value))] # [unsafe (method_family = none)] pub unsafe fn value (& self) -> c_double ;) ; }
};
}
