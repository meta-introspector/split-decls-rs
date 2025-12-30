// Generated macro for impl_eq (macro)
macro_rules! Depcrate_stringimpl_eq {
() => {
// Module: crate::string
// Provides: {"impl_eq"}
// Dependencies: {}
macro_rules ! impl_eq { ($ lhs : ty , $ rhs : ty) => { # [stable (feature = "rust1" , since = "1.0.0")] # [allow (unused_lifetimes)] impl <'a , 'b > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { PartialEq :: eq (& self [..] , & other [..]) } # [inline] fn ne (& self , other : &$ rhs) -> bool { PartialEq :: ne (& self [..] , & other [..]) } } # [stable (feature = "rust1" , since = "1.0.0")] # [allow (unused_lifetimes)] impl <'a , 'b > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { PartialEq :: eq (& self [..] , & other [..]) } # [inline] fn ne (& self , other : &$ lhs) -> bool { PartialEq :: ne (& self [..] , & other [..]) } } } ; }
};
}
