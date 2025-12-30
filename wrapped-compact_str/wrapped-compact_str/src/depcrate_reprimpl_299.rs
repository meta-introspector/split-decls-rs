// Generated macro for impl_299 (impl)
macro_rules! Depcrate_reprimpl_299 {
() => {
// Module: crate::repr
// Provides: {"impl_299"}
// Dependencies: {}
impl < 'a > Extend < Cow < 'a , str > > for Repr { fn extend < T : IntoIterator < Item = Cow < 'a , str > > > (& mut self , iter : T) { iter . into_iter () . for_each (move | s | self . push_str (& s)) ; } }
};
}
