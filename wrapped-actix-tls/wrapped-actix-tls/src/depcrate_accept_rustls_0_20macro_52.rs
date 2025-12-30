// Generated macro for macro_52 (macro)
macro_rules! Depcrate_accept_rustls_0_20macro_52 {
() => {
// Module: crate::accept::rustls_0_20
// Provides: {"macro_52"}
// Dependencies: {}
pin_project ! { # [doc = " Accept future for Rustls service."] # [doc (hidden)] pub struct AcceptFut < IO : ActixStream > { fut : Accept < IO >, # [pin] timeout : Sleep , _guard : CounterGuard , } }
};
}
