// Generated macro for impl_167 (impl)
macro_rules! Depcrate_coord_ranged1d_combinators_linspaceimpl_167 {
() => {
// Module: crate::coord::ranged1d::combinators::linspace
// Provides: {"impl_167"}
// Dependencies: {}
impl < T , R , S , RM > ValueFormatter < T > for Linspace < R , S , RM > where R : Ranged < ValueType = T > + ValueFormatter < T > , RM : LinspaceRoundingMethod < T > , T : Add < S , Output = T > + PartialOrd + Clone , S : Clone , { fn format (value : & T) -> String { R :: format (value) } }
};
}
