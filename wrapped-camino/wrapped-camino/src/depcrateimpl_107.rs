// Generated macro for impl_107 (impl)
macro_rules! Depcrateimpl_107 {
() => {
// Module: crate
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a > From < Utf8PathBuf > for Cow < 'a , Path > { fn from (path : Utf8PathBuf) -> Cow < 'a , Path > { PathBuf :: from (path) . into () } }
};
}
