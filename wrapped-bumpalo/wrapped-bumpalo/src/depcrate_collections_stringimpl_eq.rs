// Generated macro for impl_eq (macro)
macro_rules! Depcrate_collections_stringimpl_eq {
() => {
// Module: crate::collections::string
// Provides: {"impl_eq"}
// Dependencies: {}
macro_rules ! impl_eq { ($ lhs : ty , $ rhs : ty) => { impl <'a , 'bump > PartialEq <$ rhs > for $ lhs { # [inline] fn eq (& self , other : &$ rhs) -> bool { PartialEq :: eq (& self [..] , & other [..]) } } impl <'a , 'b , 'bump > PartialEq <$ lhs > for $ rhs { # [inline] fn eq (& self , other : &$ lhs) -> bool { PartialEq :: eq (& self [..] , & other [..]) } } } ; }
};
}
