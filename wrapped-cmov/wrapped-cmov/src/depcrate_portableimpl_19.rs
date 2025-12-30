// Generated macro for impl_19 (impl)
macro_rules! Depcrate_portableimpl_19 {
() => {
// Module: crate::portable
// Provides: {"impl_19"}
// Dependencies: {}
impl Cmov for u64 { # [inline] fn cmovnz (& mut self , value : & Self , condition : Condition) { let mask = is_non_zero (condition) . wrapping_sub (1) ; * self = (* self & mask) | (* value & ! mask) ; } # [inline] fn cmovz (& mut self , value : & Self , condition : Condition) { let mask = (1 ^ is_non_zero (condition)) . wrapping_sub (1) ; * self = (* self & mask) | (* value & ! mask) ; } }
};
}
