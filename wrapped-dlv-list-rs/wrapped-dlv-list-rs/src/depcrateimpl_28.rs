// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a , T > Extend < & 'a T > for VecList < T > where T : 'a + Copy , { fn extend < Iter > (& mut self , iter : Iter) where Iter : IntoIterator < Item = & 'a T > , { self . extend (iter . into_iter () . copied ()) ; } }
};
}
