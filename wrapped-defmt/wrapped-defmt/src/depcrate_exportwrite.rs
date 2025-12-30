// Generated macro for write (function)
macro_rules! Depcrate_exportwrite {
() => {
// Module: crate::export
// Provides: {"write"}
// Dependencies: {}
# [cfg (not (feature = "unstable-test"))] # [inline (always)] pub fn write (bytes : & [u8]) { extern "Rust" { fn _defmt_write (bytes : & [u8]) ; } unsafe { _defmt_write (bytes) } }
};
}
