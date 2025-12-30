// Generated macro for impl_17 (impl)
macro_rules! Depcrate_aioimpl_17 {
() => {
// Module: crate::aio
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'a > Source < aio :: AioRead < 'a > > { # [doc = " Asynchronously read from a file."] pub fn read_at (fd : BorrowedFd < 'a > , offs : u64 , buf : & 'a mut [u8] , prio : i32 ,) -> Self { let inner = aio :: AioRead :: new (fd , offs as off_t , buf , prio , SigevNotify :: SigevNone ,) ; Source { inner } } }
};
}
