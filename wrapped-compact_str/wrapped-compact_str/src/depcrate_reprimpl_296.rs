// Generated macro for impl_296 (impl)
macro_rules! Depcrate_reprimpl_296 {
() => {
// Module: crate::repr
// Provides: {"impl_296"}
// Dependencies: {}
impl < 'a > Extend < & 'a char > for Repr { fn extend < T : IntoIterator < Item = & 'a char > > (& mut self , iter : T) { self . extend (iter . into_iter () . copied ()) ; } }
};
}
