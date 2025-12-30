// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl < T > FromIterator < T > for Arena < T > { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = T > , { Arena { data : Vec :: from_iter (iter) } } }
};
}
