// Generated macro for NSUserActivityCoreNFC (trait)
macro_rules! Depcrate_generatedNSUserActivityCoreNFC {
() => {
// Module: crate::generated
// Provides: {"NSUserActivityCoreNFC"}
// Dependencies: {}
# [doc = " Category \"CoreNFC\" on [`NSUserActivity`]."] # [doc (alias = "CoreNFC")] pub unsafe trait NSUserActivityCoreNFC : ClassType + Sized + private_NSUserActivityCoreNFC :: Sealed { extern_methods ! (# [doc = " The NFC NDEF message with an Universal Link object that triggers the application launch."] # [unsafe (method (ndefMessagePayload))] # [unsafe (method_family = none)] unsafe fn ndefMessagePayload (& self) -> Retained < NFCNDEFMessage >;) ; }
};
}
