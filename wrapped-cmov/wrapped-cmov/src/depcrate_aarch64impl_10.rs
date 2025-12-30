// Generated macro for impl_10 (impl)
macro_rules! Depcrate_aarch64impl_10 {
() => {
// Module: crate::aarch64
// Provides: {"impl_10"}
// Dependencies: {}
impl Cmov for u64 { # [inline] fn cmovnz (& mut self , value : & Self , condition : Condition) { csel ! ("csel {1:x}, {2:x}, {3:x}, NE" , self , value , condition) ; } # [inline] fn cmovz (& mut self , value : & Self , condition : Condition) { csel ! ("csel {1:x}, {2:x}, {3:x}, EQ" , self , value , condition) ; } }
};
}
