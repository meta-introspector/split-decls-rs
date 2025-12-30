// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
unsafe impl defmt :: Logger for Logger { fn acquire () { RTT_ENCODER . acquire () ; } unsafe fn write (bytes : & [u8]) { unsafe { RTT_ENCODER . write (bytes) ; } } unsafe fn flush () { unsafe { RTT_ENCODER . flush () ; } } unsafe fn release () { unsafe { RTT_ENCODER . release () ; } } }
};
}
