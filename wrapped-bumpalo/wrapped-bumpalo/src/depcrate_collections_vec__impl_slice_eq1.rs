// Generated macro for __impl_slice_eq1 (macro)
macro_rules! Depcrate_collections_vec__impl_slice_eq1 {
() => {
// Module: crate::collections::vec
// Provides: {"__impl_slice_eq1"}
// Dependencies: {}
macro_rules ! __impl_slice_eq1 { ($ Lhs : ty , $ Rhs : ty) => { __impl_slice_eq1 ! { $ Lhs , $ Rhs , Sized } } ; ($ Lhs : ty , $ Rhs : ty , $ Bound : ident) => { impl <'a , 'b , A : $ Bound , B > PartialEq <$ Rhs > for $ Lhs where A : PartialEq < B >, { # [inline] fn eq (& self , other : &$ Rhs) -> bool { self [..] == other [..] } } } ; }
};
}
