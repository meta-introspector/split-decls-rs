// Generated macro for impl_19 (impl)
macro_rules! Depcrate_aioimpl_19 {
() => {
// Module: crate::aio
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a > Source < aio :: AioWrite < 'a > > { # [doc = " Asynchronously write to a file."] pub fn write_at (fd : BorrowedFd < 'a > , offs : u64 , buf : & 'a [u8] , prio : i32 ,) -> Self { let inner = aio :: AioWrite :: new (fd , offs as off_t , buf , prio , SigevNotify :: SigevNone ,) ; Source { inner } } }
};
}
