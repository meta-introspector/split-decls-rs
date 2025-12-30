// Generated macro for impl_15 (impl)
macro_rules! Depcrate_portableimpl_15 {
() => {
// Module: crate::portable
// Provides: {"impl_15"}
// Dependencies: {}
impl Cmov for u16 { # [inline] fn cmovnz (& mut self , value : & Self , condition : Condition) { let mut tmp = * self as u64 ; tmp . cmovnz (& (* value as u64) , condition) ; * self = tmp as u16 ; } # [inline] fn cmovz (& mut self , value : & Self , condition : Condition) { let mut tmp = * self as u64 ; tmp . cmovz (& (* value as u64) , condition) ; * self = tmp as u16 ; } }
};
}
