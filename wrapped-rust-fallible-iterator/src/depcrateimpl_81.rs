// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl < T , I : iter :: Iterator < Item = T > > From < I > for IntoFallible < I > { fn from (value : I) -> Self { Self (value) } }
};
}
