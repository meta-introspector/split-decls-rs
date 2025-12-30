// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < T > From < COption < T > > for Option < T > { fn from (coption : COption < T >) -> Self { match coption { COption :: Some (value) => Some (value) , COption :: None => None , } } }
};
}
