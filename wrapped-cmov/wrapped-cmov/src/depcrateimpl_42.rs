// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < T : CmovEq > CmovEq for [T] { fn cmoveq (& self , rhs : & Self , input : Condition , output : & mut Condition) { let mut tmp = 1u8 ; self . cmovne (rhs , 0u8 , & mut tmp) ; tmp . cmoveq (& 1 , input , output) ; } fn cmovne (& self , rhs : & Self , input : Condition , output : & mut Condition) { if self . len () != rhs . len () { * output = input ; return ; } for (a , b) in self . iter () . zip (rhs . iter ()) { a . cmovne (b , input , output) ; } } }
};
}
