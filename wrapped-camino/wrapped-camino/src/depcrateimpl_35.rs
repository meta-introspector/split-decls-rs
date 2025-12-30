// Generated macro for impl_35 (impl)
macro_rules! Depcrateimpl_35 {
() => {
// Module: crate
// Provides: {"impl_35"}
// Dependencies: {}
impl < P : AsRef < Utf8Path > > Extend < P > for Utf8PathBuf { fn extend < I : IntoIterator < Item = P > > (& mut self , iter : I) { for path in iter { self . push (path) ; } } }
};
}
