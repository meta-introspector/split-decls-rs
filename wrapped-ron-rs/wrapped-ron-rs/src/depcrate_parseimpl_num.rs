// Generated macro for impl_num (macro)
macro_rules! Depcrate_parseimpl_num {
() => {
// Module: crate::parse
// Provides: {"impl_num"}
// Dependencies: {}
macro_rules ! impl_num { ($ ty : ty) => { impl Num for $ ty { fn from_u8 (x : u8) -> Self { x as $ ty } fn checked_mul_ext (& mut self , x : u8) -> bool { match self . checked_mul (Self :: from_u8 (x)) { Some (n) => { * self = n ; false } None => true , } } fn checked_add_ext (& mut self , x : u8) -> bool { match self . checked_add (Self :: from_u8 (x)) { Some (n) => { * self = n ; false } None => true , } } fn checked_sub_ext (& mut self , x : u8) -> bool { match self . checked_sub (Self :: from_u8 (x)) { Some (n) => { * self = n ; false } None => true , } } } } ; ($ ($ tys : ty) *) => { $ (impl_num ! ($ tys) ;) * } ; }
};
}
