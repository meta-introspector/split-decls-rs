// Generated macro for impl_335 (impl)
macro_rules! Depcrate_generatedimpl_335 {
() => {
// Module: crate::generated
// Provides: {"impl_335"}
// Dependencies: {}
impl SRElectrocardiogramData { extern_methods ! (# [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [unsafe (method (new))] # [unsafe (method_family = new)] pub unsafe fn new () -> Retained < Self >; # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (flags))] # [unsafe (method_family = none)] pub unsafe fn flags (& self) -> SRElectrocardiogramDataFlags ; # [doc = " Value of the ECG AC data in microvolts"] # [doc = ""] # [doc = " This property is not atomic."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This might not be thread-safe."] # [unsafe (method (value))] # [unsafe (method_family = none)] pub unsafe fn value (& self) -> Retained < NSMeasurement < NSUnitElectricPotentialDifference >>;) ; }
};
}
