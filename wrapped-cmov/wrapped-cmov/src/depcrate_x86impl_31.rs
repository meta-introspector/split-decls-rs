// Generated macro for impl_31 (impl)
macro_rules! Depcrate_x86impl_31 {
() => {
// Module: crate::x86
// Provides: {"impl_31"}
// Dependencies: {}
# [cfg (target_arch = "x86")] impl Cmov for u64 { # [inline] fn cmovnz (& mut self , value : & Self , condition : Condition) { let mut lo = (* self & u32 :: MAX as u64) as u32 ; let mut hi = (* self >> 32) as u32 ; lo . cmovnz (& ((* value & u32 :: MAX as u64) as u32) , condition) ; hi . cmovnz (& ((* value >> 32) as u32) , condition) ; * self = (lo as u64) | (hi as u64) << 32 ; } # [inline] fn cmovz (& mut self , value : & Self , condition : Condition) { let mut lo = (* self & u32 :: MAX as u64) as u32 ; let mut hi = (* self >> 32) as u32 ; lo . cmovz (& ((* value & u32 :: MAX as u64) as u32) , condition) ; hi . cmovz (& ((* value >> 32) as u32) , condition) ; * self = (lo as u64) | (hi as u64) << 32 ; } }
};
}
