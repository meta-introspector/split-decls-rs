// Generated macro for impl_153 (impl)
macro_rules! Depcrate_generatedimpl_153 {
() => {
// Module: crate::generated
// Provides: {"impl_153"}
// Dependencies: {}
impl NFCVASResponse { extern_methods ! (# [doc = " Response APDU status."] # [unsafe (method (status))] # [unsafe (method_family = none)] pub unsafe fn status (& self) -> NFCVASErrorCode ; # [doc = " VAS data."] # [unsafe (method (vasData))] # [unsafe (method_family = none)] pub unsafe fn vasData (& self) -> Retained < NSData >; # [doc = " Mobile token value."] # [unsafe (method (mobileToken))] # [unsafe (method_family = none)] pub unsafe fn mobileToken (& self) -> Retained < NSData >;) ; }
};
}
