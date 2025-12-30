// Generated macro for macro_78 (macro)
macro_rules! Depcrate_accept_rustls_0_21macro_78 {
() => {
// Module: crate::accept::rustls_0_21
// Provides: {"macro_78"}
// Dependencies: {}
pin_project ! { # [doc = " Accept future for Rustls service."] # [doc (hidden)] pub struct AcceptFut < IO : ActixStream > { fut : Accept < IO >, # [pin] timeout : Sleep , _guard : CounterGuard , } }
};
}
