// Generated macro for impl_cmp_os_str (macro)
macro_rules! Depcrateimpl_cmp_os_str {
() => {
// Module: crate
// Provides: {"impl_cmp_os_str"}
// Dependencies: {}
macro_rules ! impl_cmp_os_str { ($ lhs : ty , $ rhs : ty) => { # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { < Path as PartialEq >:: eq (self . as_ref () , other . as_ref ()) } } # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { < Path as PartialEq >:: eq (self . as_ref () , other . as_ref ()) } } # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialOrd <$ rhs > for $ lhs { # [inline] fn partial_cmp (& self , other : &$ rhs) -> Option < std :: cmp :: Ordering > { < Path as PartialOrd >:: partial_cmp (self . as_ref () , other . as_ref ()) } } # [allow (clippy :: extra_unused_lifetimes)] impl <'a , 'b > PartialOrd <$ lhs > for $ rhs { # [inline] fn partial_cmp (& self , other : &$ lhs) -> Option < std :: cmp :: Ordering > { < Path as PartialOrd >:: partial_cmp (self . as_ref () , other . as_ref ()) } } } ; }
};
}
