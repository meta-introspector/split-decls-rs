// Generated macro for CONTROLS (const)
macro_rules! Depcrate_ascii_setCONTROLS {
() => {
// Module: crate::ascii_set
// Provides: {"CONTROLS"}
// Dependencies: {}
# [doc = " The set of 0x00\u{a0}to 0x1F (C0 controls), and 0x7F (DEL)."] # [doc = ""] # [doc = " Note that this includes the newline and tab characters, but not the space 0x20."] # [doc = ""] # [doc = " <https://url.spec.whatwg.org/#c0-control-percent-encode-set>"] pub const CONTROLS : & AsciiSet = & AsciiSet { mask : [! 0_u32 , 0 , 0 , 1 << (0x7F_u32 % 32) ,] , } ;
};
}
