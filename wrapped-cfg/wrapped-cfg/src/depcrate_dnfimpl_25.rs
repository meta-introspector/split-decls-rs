// Generated macro for impl_25 (impl)
macro_rules! Depcrate_dnfimpl_25 {
() => {
// Module: crate::dnf
// Provides: {"impl_25"}
// Dependencies: {}
impl fmt :: Display for Literal { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . negate { write ! (f , "not(") ? ; } match & self . var { Some (var) => var . fmt (f) ? , None => f . write_str ("<invalid>") ? , } if self . negate { f . write_char (')') ? ; } Ok (()) } }
};
}
