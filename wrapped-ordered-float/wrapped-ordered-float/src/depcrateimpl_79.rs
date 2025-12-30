// Generated macro for impl_79 (impl)
macro_rules! Depcrateimpl_79 {
() => {
// Module: crate
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a , T : FloatCore + Product + 'a > Product < & 'a OrderedFloat < T > > for OrderedFloat < T > { # [inline] fn product < I : Iterator < Item = & 'a OrderedFloat < T > > > (iter : I) -> Self { iter . cloned () . product () } }
};
}
