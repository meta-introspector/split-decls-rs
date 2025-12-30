// Generated macro for impl_18 (impl)
macro_rules! Depcrate_aioimpl_18 {
() => {
// Module: crate::aio
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a > Source < aio :: AioReadv < 'a > > { # [doc = " Asynchronously read from a file to a scatter/gather list of buffers."] # [doc = ""] # [doc = " Requires FreeBSD 13.0 or later."] pub fn readv_at (fd : BorrowedFd < 'a > , offs : u64 , bufs : & mut [IoSliceMut < 'a >] , prio : i32 ,) -> Self { let inner = aio :: AioReadv :: new (fd , offs as off_t , bufs , prio , SigevNotify :: SigevNone ,) ; Source { inner } } }
};
}
