// Generated macro for impl_29 (impl)
macro_rules! Depcrate_x86impl_29 {
() => {
// Module: crate::x86
// Provides: {"impl_29"}
// Dependencies: {}
impl Cmov for u32 { # [inline] fn cmovnz (& mut self , value : & Self , condition : Condition) { cmov ! ("cmovnz {1:e}, {2:e}" , self , value , condition) ; } # [inline] fn cmovz (& mut self , value : & Self , condition : Condition) { cmov ! ("cmovz {1:e}, {2:e}" , self , value , condition) ; } }
};
}
