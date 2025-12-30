// Generated macro for impl_34 (impl)
macro_rules! Depcrate_x86impl_34 {
() => {
// Module: crate::x86
// Provides: {"impl_34"}
// Dependencies: {}
# [cfg (target_arch = "x86_64")] impl CmovEq for u64 { # [inline] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { cmov_eq ! ("xor {0:r}, {1:r}" , "cmovz {2:r}, {3:r}" , self , rhs , input , output) ; } # [inline] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { cmov_eq ! ("xor {0:r}, {1:r}" , "cmovnz {2:r}, {3:r}" , self , rhs , input , output) ; } }
};
}
