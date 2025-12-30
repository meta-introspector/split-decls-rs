// Generated macro for to_llvm_tls_model (function)
macro_rules! Depcrate_contextto_llvm_tls_model {
() => {
// Module: crate::context
// Provides: {"to_llvm_tls_model"}
// Dependencies: {}
fn to_llvm_tls_model (tls_model : TlsModel) -> llvm :: ThreadLocalMode { match tls_model { TlsModel :: GeneralDynamic => llvm :: ThreadLocalMode :: GeneralDynamic , TlsModel :: LocalDynamic => llvm :: ThreadLocalMode :: LocalDynamic , TlsModel :: InitialExec => llvm :: ThreadLocalMode :: InitialExec , TlsModel :: LocalExec => llvm :: ThreadLocalMode :: LocalExec , TlsModel :: Emulated => llvm :: ThreadLocalMode :: GeneralDynamic , } }
};
}
