// Generated macro for impl_86 (impl)
macro_rules! Depcrateimpl_86 {
() => {
// Module: crate
// Provides: {"impl_86"}
// Dependencies: {}
impl < T : ? Sized + AsRef < str > > From < & T > for Utf8PathBuf { fn from (s : & T) -> Utf8PathBuf { Utf8PathBuf :: from (s . as_ref () . to_owned ()) } }
};
}
