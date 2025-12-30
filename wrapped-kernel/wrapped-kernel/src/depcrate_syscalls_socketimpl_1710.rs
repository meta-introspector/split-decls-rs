// Generated macro for impl_1710 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1710 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1710"}
// Dependencies: {}
impl sockaddr { pub unsafe fn sa_family (ptr : * const Self) -> Result < Af , TryFromPrimitiveError < Af > > { let sa_family = unsafe { (* ptr) . sa_family } ; Af :: try_from (sa_family) } pub unsafe fn as_ref (ptr : & * const Self) -> Result < sockaddrRef < '_ > , TryFromPrimitiveError < Af > > { let ptr = * ptr ; let sa_family = unsafe { Self :: sa_family (ptr) ? } ; let ret = match sa_family { Af :: Unspec => sockaddrRef :: sockaddr (unsafe { & * ptr }) , Af :: Inet => sockaddrRef :: sockaddr_in (unsafe { & * ptr . cast () }) , Af :: Inet6 => sockaddrRef :: sockaddr_in6 (unsafe { & * ptr . cast () }) , Af :: Unix => sockaddrRef :: sockaddr_un (unsafe { & * ptr . cast () }) , # [cfg (feature = "vsock")] Af :: Vsock => sockaddrRef :: sockaddr_vm (unsafe { & * ptr . cast () }) , } ; Ok (ret) } pub unsafe fn as_box (ptr : * mut Self) -> Result < sockaddrBox , TryFromPrimitiveError < Af > > { let sa_family = unsafe { Self :: sa_family (ptr) ? } ; let ret = match sa_family { Af :: Unspec => sockaddrBox :: sockaddr (unsafe { Box :: from_raw (ptr) }) , Af :: Inet => sockaddrBox :: sockaddr_in (unsafe { Box :: from_raw (ptr . cast ()) }) , Af :: Inet6 => sockaddrBox :: sockaddr_in6 (unsafe { Box :: from_raw (ptr . cast ()) }) , Af :: Unix => sockaddrBox :: sockaddr_un (unsafe { Box :: from_raw (ptr . cast ()) }) , # [cfg (feature = "vsock")] Af :: Vsock => sockaddrBox :: sockaddr_vm (unsafe { Box :: from_raw (ptr . cast ()) }) , } ; Ok (ret) } }
};
}
