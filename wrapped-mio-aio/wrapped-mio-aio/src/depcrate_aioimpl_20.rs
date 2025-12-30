// Generated macro for impl_20 (impl)
macro_rules! Depcrate_aioimpl_20 {
() => {
// Module: crate::aio
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a > Source < aio :: AioWritev < 'a > > { # [doc = " Asynchronously write to a file to a scatter/gather list of buffers."] # [doc = ""] # [doc = " Requires FreeBSD 13.0 or later."] pub fn writev_at (fd : BorrowedFd < 'a > , offs : u64 , bufs : & [IoSlice < 'a >] , prio : i32 ,) -> Self { let inner = aio :: AioWritev :: new (fd , offs as off_t , bufs , prio , SigevNotify :: SigevNone ,) ; Source { inner } } }
};
}
