// Generated macro for impl_87 (impl)
macro_rules! Depcrateimpl_87 {
() => {
// Module: crate
// Provides: {"impl_87"}
// Dependencies: {}
impl < T : ? Sized + AsRef < str > > From < & T > for Box < Utf8Path > { fn from (s : & T) -> Box < Utf8Path > { Utf8PathBuf :: from (s) . into_boxed_path () } }
};
}
