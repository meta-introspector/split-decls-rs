// Generated macro for impl_21 (impl)
macro_rules! Depcrate_dnfimpl_21 {
() => {
// Module: crate::dnf
// Provides: {"impl_21"}
// Dependencies: {}
impl fmt :: Display for DnfExpr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . conjunctions . len () != 1 { f . write_str ("any(") ? ; } for (i , conj) in self . conjunctions . iter () . enumerate () { if i != 0 { f . write_str (", ") ? ; } conj . fmt (f) ? ; } if self . conjunctions . len () != 1 { f . write_char (')') ? ; } Ok (()) } }
};
}
