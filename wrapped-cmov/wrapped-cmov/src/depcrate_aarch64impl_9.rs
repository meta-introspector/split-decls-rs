// Generated macro for impl_9 (impl)
macro_rules! Depcrate_aarch64impl_9 {
() => {
// Module: crate::aarch64
// Provides: {"impl_9"}
// Dependencies: {}
impl CmovEq for u32 { # [inline] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { csel_eq ! ("csel {3:w}, {4:w}, {5:w}, NE" , self , rhs , input , output) ; } # [inline] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { csel_eq ! ("csel {3:w}, {4:w}, {5:w}, EQ" , self , rhs , input , output) ; } }
};
}
