// Generated macro for impl_297 (impl)
macro_rules! Depcrate_reprimpl_297 {
() => {
// Module: crate::repr
// Provides: {"impl_297"}
// Dependencies: {}
impl < 'a > Extend < & 'a str > for Repr { fn extend < T : IntoIterator < Item = & 'a str > > (& mut self , iter : T) { iter . into_iter () . for_each (| s | self . push_str (s)) ; } }
};
}
