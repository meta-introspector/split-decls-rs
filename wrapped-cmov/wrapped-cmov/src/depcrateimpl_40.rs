// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl Cmov for u128 { # [inline] fn cmovnz (& mut self , value : & Self , condition : Condition) { let mut lo = (* self & u64 :: MAX as u128) as u64 ; let mut hi = (* self >> 64) as u64 ; lo . cmovnz (& ((* value & u64 :: MAX as u128) as u64) , condition) ; hi . cmovnz (& ((* value >> 64) as u64) , condition) ; * self = (lo as u128) | ((hi as u128) << 64) ; } # [inline] fn cmovz (& mut self , value : & Self , condition : Condition) { let mut lo = (* self & u64 :: MAX as u128) as u64 ; let mut hi = (* self >> 64) as u64 ; lo . cmovz (& ((* value & u64 :: MAX as u128) as u64) , condition) ; hi . cmovz (& ((* value >> 64) as u64) , condition) ; * self = (lo as u128) | ((hi as u128) << 64) ; } }
};
}
