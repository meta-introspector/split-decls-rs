// Generated macro for impl_100 (impl)
macro_rules! Depcrateimpl_100 {
() => {
// Module: crate
// Provides: {"impl_100"}
// Dependencies: {}
# [cfg (feature = "builder")] impl PackageBuilder { # [doc = " Construct a new `PackageBuilder` with all required fields."] pub fn new (name : impl Into < PackageName > , version : impl Into < Version > , id : impl Into < PackageId > , path : impl Into < Utf8PathBuf > ,) -> Self { Self :: default () . name (name) . version (version) . id (id) . manifest_path (path) } }
};
}
