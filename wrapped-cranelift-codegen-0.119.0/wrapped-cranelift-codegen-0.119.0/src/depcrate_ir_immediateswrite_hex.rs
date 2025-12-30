// Generated macro for write_hex (function)
macro_rules! Depcrate_ir_immediateswrite_hex {
() => {
// Module: crate::ir::immediates
// Provides: {"write_hex"}
// Dependencies: {}
# [doc = " Hexadecimal with a multiple of 4 digits and group separators:"] # [doc = ""] # [doc = "   0xfff0"] # [doc = "   0x0001_ffff"] # [doc = "   0xffff_ffff_fff8_4400"] # [doc = ""] fn write_hex (x : u64 , f : & mut Formatter) -> fmt :: Result { let mut pos = (64 - x . leading_zeros () - 1) & 0xf0 ; write ! (f , "0x{:04x}" , (x >> pos) & 0xffff) ? ; while pos > 0 { pos -= 16 ; write ! (f , "_{:04x}" , (x >> pos) & 0xffff) ? ; } Ok (()) }
};
}
