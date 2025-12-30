// Generated macro for impl_partial_eq_n (macro)
macro_rules! Depcrate_implsimpl_partial_eq_n {
() => {
// Module: crate::impls
// Provides: {"impl_partial_eq_n"}
// Dependencies: {}
macro_rules ! impl_partial_eq_n { ($ lhs : ty , $ rhs : ty) => { impl <'a , const N : usize > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { let other : & [u8] = other . as_ref () ; PartialEq :: eq (self . as_bytes () , other) } } impl <'a , const N : usize > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { let this : & [u8] = self . as_ref () ; PartialEq :: eq (this , other . as_bytes ()) } } } ; }
};
}
