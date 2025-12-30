// Generated macro for impl_300 (impl)
macro_rules! Depcrate_reprimpl_300 {
() => {
// Module: crate::repr
// Provides: {"impl_300"}
// Dependencies: {}
impl Extend < String > for Repr { fn extend < T : IntoIterator < Item = String > > (& mut self , iter : T) { iter . into_iter () . for_each (move | s | self . push_str (& s)) ; } }
};
}
