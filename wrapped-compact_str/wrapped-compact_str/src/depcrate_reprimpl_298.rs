// Generated macro for impl_298 (impl)
macro_rules! Depcrate_reprimpl_298 {
() => {
// Module: crate::repr
// Provides: {"impl_298"}
// Dependencies: {}
impl Extend < Box < str > > for Repr { fn extend < T : IntoIterator < Item = Box < str > > > (& mut self , iter : T) { iter . into_iter () . for_each (move | s | self . push_str (& s)) ; } }
};
}
