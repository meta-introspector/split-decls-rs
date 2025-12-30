// Generated macro for impl_101 (impl)
macro_rules! Depcrateimpl_101 {
() => {
// Module: crate
// Provides: {"impl_101"}
// Dependencies: {}
impl Package { # [doc = " Full path to the license file if one is present in the manifest"] pub fn license_file (& self) -> Option < Utf8PathBuf > { self . license_file . as_ref () . map (| file | { self . manifest_path . parent () . unwrap_or (& self . manifest_path) . join (file) }) } # [doc = " Full path to the readme file if one is present in the manifest"] pub fn readme (& self) -> Option < Utf8PathBuf > { self . readme . as_ref () . map (| file | { self . manifest_path . parent () . unwrap_or (& self . manifest_path) . join (file) }) } }
};
}
