// Generated macro for impl_partial_ord_n (macro)
macro_rules! Depcrate_implsimpl_partial_ord_n {
() => {
// Module: crate::impls
// Provides: {"impl_partial_ord_n"}
// Dependencies: {}
macro_rules ! impl_partial_ord_n { ($ lhs : ty , $ rhs : ty) => { impl <'a , const N : usize > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < Ordering > { let other : & [u8] = other . as_ref () ; PartialOrd :: partial_cmp (self . as_bytes () , other) } } impl <'a , const N : usize > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < Ordering > { let this : & [u8] = self . as_ref () ; PartialOrd :: partial_cmp (this , other . as_bytes ()) } } } ; }
};
}
