// Generated macro for impl_16 (impl)
macro_rules! Depcrate_aioimpl_16 {
() => {
// Module: crate::aio
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'a > Source < aio :: AioFsync < 'a > > { # [doc = " Asynchronously fsync a file."] pub fn fsync (fd : BorrowedFd < 'a > , mode : AioFsyncMode , prio : i32) -> Self { let inner = aio :: AioFsync :: new (fd , mode , prio , SigevNotify :: SigevNone) ; Source { inner } } }
};
}
