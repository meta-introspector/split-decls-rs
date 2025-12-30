// Generated macro for impl_508 (impl)
macro_rules! Depcrateimpl_508 {
() => {
// Module: crate
// Provides: {"impl_508"}
// Dependencies: {}
impl Extend < CompactString > for String { fn extend < T : IntoIterator < Item = CompactString > > (& mut self , iter : T) { for s in iter { self . push_str (& s) ; } } }
};
}
