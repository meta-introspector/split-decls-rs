// Generated macro for impl_322 (impl)
macro_rules! Depcrate_generatedimpl_322 {
() => {
// Module: crate::generated
// Provides: {"impl_322"}
// Dependencies: {}
impl SRElectrocardiogramSession { extern_methods ! (# [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [unsafe (method (new))] # [unsafe (method_family = new)] pub unsafe fn new () -> Retained < Self >; # [doc = " The state of the ECG session when the sample was recorded"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (state))] # [unsafe (method_family = none)] pub unsafe fn state (& self) -> SRElectrocardiogramSessionState ; # [doc = " The type of session guidance during the the ECG session"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (sessionGuidance))] # [unsafe (method_family = none)] pub unsafe fn sessionGuidance (& self) -> SRElectrocardiogramSessionGuidance ; # [doc = " Used to tie samples across multiple"] # [doc = " `SRFetchResult`s to the same session"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (identifier))] # [unsafe (method_family = none)] pub unsafe fn identifier (& self) -> Retained < NSString >;) ; }
};
}
