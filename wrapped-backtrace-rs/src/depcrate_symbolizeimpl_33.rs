// Generated macro for impl_33 (impl)
macro_rules! Depcrate_symbolizeimpl_33 {
() => {
// Module: crate::symbolize
// Provides: {"impl_33"}
// Dependencies: {}
impl fmt :: Debug for Symbol { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = f . debug_struct ("Symbol") ; if let Some (name) = self . name () { d . field ("name" , & name) ; } if let Some (addr) = self . addr () { d . field ("addr" , & addr) ; } # [cfg (feature = "std")] { if let Some (filename) = self . filename () { d . field ("filename" , & filename) ; } } if let Some (lineno) = self . lineno () { d . field ("lineno" , & lineno) ; } d . finish () } }
};
}
