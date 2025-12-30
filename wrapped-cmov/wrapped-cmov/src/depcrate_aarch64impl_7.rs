// Generated macro for impl_7 (impl)
macro_rules! Depcrate_aarch64impl_7 {
() => {
// Module: crate::aarch64
// Provides: {"impl_7"}
// Dependencies: {}
impl CmovEq for u16 { # [inline] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { csel_eq ! ("csel {3:w}, {4:w}, {5:w}, NE" , self , rhs , input , output) ; } # [inline] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { csel_eq ! ("csel {3:w}, {4:w}, {5:w}, EQ" , self , rhs , input , output) ; } }
};
}
