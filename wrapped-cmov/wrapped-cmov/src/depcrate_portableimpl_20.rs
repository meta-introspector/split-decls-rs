// Generated macro for impl_20 (impl)
macro_rules! Depcrate_portableimpl_20 {
() => {
// Module: crate::portable
// Provides: {"impl_20"}
// Dependencies: {}
impl CmovEq for u64 { # [inline] fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { output . cmovnz (& input , (self ^ rhs) as u8) ; } # [inline] fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { output . cmovz (& input , (self ^ rhs) as u8) ; } }
};
}
