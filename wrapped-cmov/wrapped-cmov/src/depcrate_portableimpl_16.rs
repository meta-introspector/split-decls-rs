// Generated macro for impl_16 (impl)
macro_rules! Depcrate_portableimpl_16 {
() => {
// Module: crate::portable
// Provides: {"impl_16"}
// Dependencies: {}
impl CmovEq for u16 { # [inline] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { (* self as u64) . cmovne (& (* rhs as u64) , input , output) ; } # [inline] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { (* self as u64) . cmoveq (& (* rhs as u64) , input , output) ; } }
};
}
