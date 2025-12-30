// Generated macro for impl_11 (impl)
macro_rules! Depcrate_aarch64impl_11 {
() => {
// Module: crate::aarch64
// Provides: {"impl_11"}
// Dependencies: {}
impl CmovEq for u64 { # [inline] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { csel_eq ! ("csel {3:w}, {4:w}, {5:w}, NE" , self , rhs , input , output) ; } # [inline] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { csel_eq ! ("csel {3:w}, {4:w}, {5:w}, EQ" , self , rhs , input , output) ; } }
};
}
