// Generated macro for impl_6 (impl)
macro_rules! Depcrate_aarch64impl_6 {
() => {
// Module: crate::aarch64
// Provides: {"impl_6"}
// Dependencies: {}
impl Cmov for u16 { # [inline] fn cmovnz (& mut self , value : & Self , condition : Condition) { csel ! ("csel {1:w}, {2:w}, {3:w}, NE" , self , value , condition) ; } # [inline] fn cmovz (& mut self , value : & Self , condition : Condition) { csel ! ("csel {1:w}, {2:w}, {3:w}, EQ" , self , value , condition) ; } }
};
}
