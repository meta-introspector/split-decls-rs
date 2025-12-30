// Generated macro for impl_77 (impl)
macro_rules! Depcrateimpl_77 {
() => {
// Module: crate
// Provides: {"impl_77"}
// Dependencies: {}
impl < 'a , T : FloatCore + Sum + 'a > Sum < & 'a OrderedFloat < T > > for OrderedFloat < T > { # [inline] fn sum < I : Iterator < Item = & 'a OrderedFloat < T > > > (iter : I) -> Self { iter . cloned () . sum () } }
};
}
