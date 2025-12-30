// Generated macro for impl_28 (impl)
macro_rules! Depcrate_x86impl_28 {
() => {
// Module: crate::x86
// Provides: {"impl_28"}
// Dependencies: {}
impl CmovEq for u16 { # [inline] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { cmov_eq ! ("xor {0:x}, {1:x}" , "cmovz {2:e}, {3:e}" , self , rhs , input , output) ; } # [inline] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { cmov_eq ! ("xor {0:x}, {1:x}" , "cmovnz {2:e}, {3:e}" , self , rhs , input , output) ; } }
};
}
