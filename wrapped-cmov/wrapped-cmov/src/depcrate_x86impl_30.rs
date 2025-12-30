// Generated macro for impl_30 (impl)
macro_rules! Depcrate_x86impl_30 {
() => {
// Module: crate::x86
// Provides: {"impl_30"}
// Dependencies: {}
impl CmovEq for u32 { # [inline] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { cmov_eq ! ("xor {0:e}, {1:e}" , "cmovz {2:e}, {3:e}" , self , rhs , input , output) ; } # [inline] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { cmov_eq ! ("xor {0:e}, {1:e}" , "cmovnz {2:e}, {3:e}" , self , rhs , input , output) ; } }
};
}
