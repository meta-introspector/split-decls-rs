// Generated macro for impl_509 (impl)
macro_rules! Depcrateimpl_509 {
() => {
// Module: crate
// Provides: {"impl_509"}
// Dependencies: {}
impl Extend < CompactString > for CompactString { fn extend < T : IntoIterator < Item = CompactString > > (& mut self , iter : T) { for s in iter { self . push_str (& s) ; } } }
};
}
