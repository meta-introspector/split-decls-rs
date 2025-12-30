// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl < T : Clone + PartialOrd + PartialEq + Eq + Debug > Ord for AnyRange < T > { fn cmp (& self , other : & Self) -> Ordering { PartialOrd :: partial_cmp (self , other) . unwrap () } }
};
}
