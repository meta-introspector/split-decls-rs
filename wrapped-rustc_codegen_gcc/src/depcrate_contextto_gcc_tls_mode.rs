// Generated macro for to_gcc_tls_mode (function)
macro_rules! Depcrate_contextto_gcc_tls_mode {
() => {
// Module: crate::context
// Provides: {"to_gcc_tls_mode"}
// Dependencies: {}
fn to_gcc_tls_mode (tls_model : TlsModel) -> gccjit :: TlsModel { match tls_model { TlsModel :: GeneralDynamic => gccjit :: TlsModel :: GlobalDynamic , TlsModel :: LocalDynamic => gccjit :: TlsModel :: LocalDynamic , TlsModel :: InitialExec => gccjit :: TlsModel :: InitialExec , TlsModel :: LocalExec => gccjit :: TlsModel :: LocalExec , TlsModel :: Emulated => gccjit :: TlsModel :: GlobalDynamic , } }
};
}
