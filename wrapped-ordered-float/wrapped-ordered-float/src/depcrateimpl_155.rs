// Generated macro for impl_155 (impl)
macro_rules! Depcrateimpl_155 {
() => {
// Module: crate
// Provides: {"impl_155"}
// Dependencies: {}
impl < 'a , T : FloatCore + Sum + 'a > Sum < & 'a NotNan < T > > for NotNan < T > { # [inline] fn sum < I : Iterator < Item = & 'a NotNan < T > > > (iter : I) -> Self { iter . cloned () . sum () } }
};
}
