// Generated macro for macro_103 (macro)
macro_rules! Depcrate_accept_rustls_0_22macro_103 {
() => {
// Module: crate::accept::rustls_0_22
// Provides: {"macro_103"}
// Dependencies: {}
pin_project ! { # [doc = " Accept future for Rustls service."] # [doc (hidden)] pub struct AcceptFut < IO : ActixStream > { fut : Accept < IO >, # [pin] timeout : Sleep , _guard : CounterGuard , } }
};
}
