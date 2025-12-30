// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
unsafe impl defmt :: Logger for Logger { fn acquire () { SEMIHOSTING_ENCODER . acquire () ; } unsafe fn flush () { } unsafe fn release () { unsafe { SEMIHOSTING_ENCODER . release () ; } } unsafe fn write (bytes : & [u8]) { unsafe { SEMIHOSTING_ENCODER . write (bytes) ; } } }
};
}
