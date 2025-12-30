// Generated macro for __impl_slice_eq1_array (macro)
macro_rules! Depcrate_collections_vec__impl_slice_eq1_array {
() => {
// Module: crate::collections::vec
// Provides: {"__impl_slice_eq1_array"}
// Dependencies: {}
macro_rules ! __impl_slice_eq1_array { ($ Lhs : ty , $ Rhs : ty) => { impl <'a , 'b , A , B , const N : usize > PartialEq <$ Rhs > for $ Lhs where A : PartialEq < B >, { # [inline] fn eq (& self , other : &$ Rhs) -> bool { self [..] == other [..] } } } ; }
};
}
