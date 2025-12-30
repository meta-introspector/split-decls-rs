// Generated macro for impl_26 (impl)
macro_rules! Depcrate_errnoimpl_26 {
() => {
// Module: crate::errno
// Provides: {"impl_26"}
// Dependencies: {}
impl TryFrom < io :: Error > for Errno { type Error = io :: Error ; fn try_from (ioerror : io :: Error) -> std :: result :: Result < Self , io :: Error > { ioerror . raw_os_error () . map (Errno :: from_raw) . ok_or (ioerror) } }
};
}
