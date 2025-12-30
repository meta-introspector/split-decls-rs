// Generated macro for impl_580 (impl)
macro_rules! Depcrate_parser_regeximpl_580 {
() => {
// Module: crate::parser::regex
// Provides: {"impl_580"}
// Dependencies: {}
impl < A > FromIterator < A > for First < A > { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = A > , { First (iter . into_iter () . next ()) } }
};
}
