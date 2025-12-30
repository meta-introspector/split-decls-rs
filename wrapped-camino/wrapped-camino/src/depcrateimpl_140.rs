// Generated macro for impl_140 (impl)
macro_rules! Depcrateimpl_140 {
() => {
// Module: crate
// Provides: {"impl_140"}
// Dependencies: {}
impl < P : AsRef < Utf8Path > > std :: iter :: FromIterator < P > for Utf8PathBuf { fn from_iter < I : IntoIterator < Item = P > > (iter : I) -> Utf8PathBuf { let mut buf = Utf8PathBuf :: new () ; buf . extend (iter) ; buf } }
};
}
