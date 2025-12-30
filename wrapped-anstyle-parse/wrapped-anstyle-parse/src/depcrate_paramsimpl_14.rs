// Generated macro for impl_14 (impl)
macro_rules! Depcrate_paramsimpl_14 {
() => {
// Module: crate::params
// Provides: {"impl_14"}
// Dependencies: {}
impl Debug for Params { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { write ! (f , "[") ? ; for (i , param) in self . iter () . enumerate () { if i != 0 { write ! (f , ";") ? ; } for (i , subparam) in param . iter () . enumerate () { if i != 0 { write ! (f , ":") ? ; } subparam . fmt (f) ? ; } } write ! (f , "]") } }
};
}
