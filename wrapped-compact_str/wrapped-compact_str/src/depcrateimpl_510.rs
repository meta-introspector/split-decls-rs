// Generated macro for impl_510 (impl)
macro_rules! Depcrateimpl_510 {
() => {
// Module: crate
// Provides: {"impl_510"}
// Dependencies: {}
impl Extend < CompactString > for Cow < '_ , str > { fn extend < T : IntoIterator < Item = CompactString > > (& mut self , iter : T) { self . to_mut () . extend (iter) ; } }
};
}
