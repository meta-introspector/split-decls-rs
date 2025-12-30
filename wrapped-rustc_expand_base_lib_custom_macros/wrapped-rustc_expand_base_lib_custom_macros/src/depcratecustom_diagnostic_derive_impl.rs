// Generated macro for custom_diagnostic_derive_impl (function)
macro_rules! Depcratecustom_diagnostic_derive_impl {
() => {
// Module: crate
// Provides: {"custom_diagnostic_derive_impl"}
// Dependencies: {}
fn custom_diagnostic_derive_impl (s : Structure) -> TokenStream { let name = & s . ast () . ident ; let expanded = quote ! { impl <'a > rustc_errors :: Diagnostic <'a , () > for # name { fn into_diag (self , dcx : rustc_errors :: DiagCtxtHandle <'a >, level : rustc_errors :: Level) -> rustc_errors :: Diag <'a , () > { let msg = format ! ("Dummy diagnostic for {}" , stringify ! (# name)) ; let mut diag = rustc_errors :: Diag :: new (dcx , level , msg) ; diag . span (self . span) ; diag } } } ; expanded . into () }
};
}
