// Generated macro for impl_17 (impl)
macro_rules! Depcrate_portableimpl_17 {
() => {
// Module: crate::portable
// Provides: {"impl_17"}
// Dependencies: {}
impl Cmov for u32 { # [inline] fn cmovnz (& mut self , value : & Self , condition : Condition) { let mut tmp = * self as u64 ; tmp . cmovnz (& (* value as u64) , condition) ; * self = tmp as u32 ; } # [inline] fn cmovz (& mut self , value : & Self , condition : Condition) { let mut tmp = * self as u64 ; tmp . cmovz (& (* value as u64) , condition) ; * self = tmp as u32 ; } }
};
}
