// Generated macro for macro_128 (macro)
macro_rules! Depcrate_accept_rustls_0_23macro_128 {
() => {
// Module: crate::accept::rustls_0_23
// Provides: {"macro_128"}
// Dependencies: {}
pin_project ! { # [doc = " Accept future for Rustls service."] # [doc (hidden)] pub struct AcceptFut < IO : ActixStream > { fut : Accept < IO >, # [pin] timeout : Sleep , _guard : CounterGuard , } }
};
}
