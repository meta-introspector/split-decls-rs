// Generated macro for ProcMacroSrv (struct)
macro_rules! DepcrateProcMacroSrv {
() => {
// Module: crate
// Provides: {"ProcMacroSrv"}
// Dependencies: {}
pub struct ProcMacroSrv < 'env > { expanders : Mutex < HashMap < Utf8PathBuf , Arc < dylib :: Expander > > > , env : & 'env EnvSnapshot , temp_dir : TempDir , }
};
}
