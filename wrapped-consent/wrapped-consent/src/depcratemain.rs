// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { unsafe { let interop = factory :: < UserConsentVerifier , IUserConsentVerifierInterop > () ? ; let window = HWND :: default () ; let operation : IAsyncOperation < UserConsentVerificationResult > = interop . RequestVerificationForWindowAsync (window , h ! ("Hello from Rust")) ? ; let result : UserConsentVerificationResult = operation . join () ? ; println ! ("{result:?}") ; Ok (()) } }
};
}
