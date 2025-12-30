// Generated macro for impl_159 (impl)
macro_rules! Depcrateimpl_159 {
() => {
// Module: crate
// Provides: {"impl_159"}
// Dependencies: {}
impl < 'a , T : FloatCore + Product + 'a > Product < & 'a NotNan < T > > for NotNan < T > { # [inline] fn product < I : Iterator < Item = & 'a NotNan < T > > > (iter : I) -> Self { iter . cloned () . product () } }
};
}
