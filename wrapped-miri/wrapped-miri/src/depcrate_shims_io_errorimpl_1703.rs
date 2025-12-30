// Generated macro for impl_1703 (impl)
macro_rules! Depcrate_shims_io_errorimpl_1703 {
() => {
// Module: crate::shims::io_error
// Provides: {"impl_1703"}
// Dependencies: {}
impl IoError { pub (crate) fn into_ntstatus (self) -> i32 { let raw = match self { HostError (e) => match e . kind () { ErrorKind :: ReadOnlyFilesystem => 0xC00000A2u32 , ErrorKind :: InvalidInput => 0xC0000098 , ErrorKind :: QuotaExceeded => 0xC000007F , ErrorKind :: PermissionDenied => 0xC0000022 , _ => 0xC0000185 , } , _ => 0xC0000185 , } ; raw . cast_signed () } }
};
}
