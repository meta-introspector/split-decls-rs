// Generated macro for impl_partial_eq_ord_cow (macro)
macro_rules! Depcrate_bstrimpl_partial_eq_ord_cow {
() => {
// Module: crate::bstr
// Provides: {"impl_partial_eq_ord_cow"}
// Dependencies: {}
macro_rules ! impl_partial_eq_ord_cow { ($ lhs : ty , $ rhs : ty) => { # [allow (unused_lifetimes)] # [unstable (feature = "bstr" , issue = "134915")] impl <'a > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { let other : & [u8] = (&** other) . as_ref () ; PartialEq :: eq (self . as_bytes () , other) } } # [allow (unused_lifetimes)] # [unstable (feature = "bstr" , issue = "134915")] impl <'a > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { let this : & [u8] = (&** self) . as_ref () ; PartialEq :: eq (this , other . as_bytes ()) } } # [allow (unused_lifetimes)] # [unstable (feature = "bstr" , issue = "134915")] impl <'a > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < Ordering > { let other : & [u8] = (&** other) . as_ref () ; PartialOrd :: partial_cmp (self . as_bytes () , other) } } # [allow (unused_lifetimes)] # [unstable (feature = "bstr" , issue = "134915")] impl <'a > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < Ordering > { let this : & [u8] = (&** self) . as_ref () ; PartialOrd :: partial_cmp (this , other . as_bytes ()) } } } ; }
};
}
