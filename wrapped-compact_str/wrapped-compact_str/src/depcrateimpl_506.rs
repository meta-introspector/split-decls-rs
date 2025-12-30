// Generated macro for impl_506 (impl)
macro_rules! Depcrateimpl_506 {
() => {
// Module: crate
// Provides: {"impl_506"}
// Dependencies: {}
impl < 'a > Extend < Cow < 'a , str > > for CompactString { fn extend < T : IntoIterator < Item = Cow < 'a , str > > > (& mut self , iter : T) { iter . into_iter () . for_each (move | s | self . push_str (& s)) ; } }
};
}
