// Generated macro for macro_27 (macro)
macro_rules! Depcrate_accept_opensslmacro_27 {
() => {
// Module: crate::accept::openssl
// Provides: {"macro_27"}
// Dependencies: {}
pin_project ! { # [doc = " Accept future for OpenSSL service."] # [doc (hidden)] pub struct AcceptFut < IO : ActixStream > { stream : Option < tokio_openssl :: SslStream < IO >>, # [pin] timeout : Sleep , _guard : CounterGuard , } }
};
}
