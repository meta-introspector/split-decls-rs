// Generated macro for MacOsFutexTimeout (enum)
macro_rules! Depcrate_shims_unix_macos_syncMacOsFutexTimeout {
() => {
// Module: crate::shims::unix::macos::sync
// Provides: {"MacOsFutexTimeout"}
// Dependencies: {}
pub enum MacOsFutexTimeout < 'a , 'tcx > { None , Relative { clock_op : & 'a OpTy < 'tcx > , timeout_op : & 'a OpTy < 'tcx > } , Absolute { clock_op : & 'a OpTy < 'tcx > , timeout_op : & 'a OpTy < 'tcx > } , }
};
}
