// Generated macro for Byte (trait)
macro_rules! Depcrate_extByte {
() => {
// Module: crate::ext
// Provides: {"Byte"}
// Dependencies: {}
# [doc = " A trait for adding some helper routines to raw bytes."] # [cfg (test)] pub (crate) trait Byte { # [doc = " Converts this byte to a `char` if it's ASCII. Otherwise panics."] fn to_char (self) -> char ; }
};
}
