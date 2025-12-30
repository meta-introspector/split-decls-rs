// Generated macro for impl_23 (impl)
macro_rules! Depcrate_dnfimpl_23 {
() => {
// Module: crate::dnf
// Provides: {"impl_23"}
// Dependencies: {}
impl fmt :: Display for Conjunction { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . literals . len () != 1 { f . write_str ("all(") ? ; } for (i , lit) in self . literals . iter () . enumerate () { if i != 0 { f . write_str (", ") ? ; } lit . fmt (f) ? ; } if self . literals . len () != 1 { f . write_str (")") ? ; } Ok (()) } }
};
}
