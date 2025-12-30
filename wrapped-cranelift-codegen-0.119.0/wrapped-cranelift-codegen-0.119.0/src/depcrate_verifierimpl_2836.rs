// Generated macro for impl_2836 (impl)
macro_rules! Depcrate_verifierimpl_2836 {
() => {
// Module: crate::verifier
// Provides: {"impl_2836"}
// Dependencies: {}
impl VerifierErrors { # [doc = " Return a new `VerifierErrors` struct."] # [inline] pub fn new () -> Self { Self (Vec :: new ()) } # [doc = " Return whether no errors were reported."] # [inline] pub fn is_empty (& self) -> bool { self . 0 . is_empty () } # [doc = " Return whether one or more errors were reported."] # [inline] pub fn has_error (& self) -> bool { ! self . 0 . is_empty () } # [doc = " Return a `VerifierStepResult` that is fatal if at least one error was reported,"] # [doc = " and non-fatal otherwise."] # [inline] pub fn as_result (& self) -> VerifierStepResult { if self . is_empty () { Ok (()) } else { Err (()) } } # [doc = " Report an error, adding it to the list of errors."] pub fn report (& mut self , error : impl Into < VerifierError >) { self . 0 . push (error . into ()) ; } # [doc = " Report a fatal error and return `Err`."] pub fn fatal (& mut self , error : impl Into < VerifierError >) -> VerifierStepResult { self . report (error) ; Err (()) } # [doc = " Report a non-fatal error and return `Ok`."] pub fn nonfatal (& mut self , error : impl Into < VerifierError >) -> VerifierStepResult { self . report (error) ; Ok (()) } }
};
}
