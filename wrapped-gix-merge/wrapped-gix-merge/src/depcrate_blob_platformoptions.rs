// Generated macro for Options (struct)
macro_rules! Depcrate_blob_platformOptions {
() => {
// Module: crate::blob::platform
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for use in [`Platform::new()`]."] # [derive (Default , Clone , PartialEq , Eq , Debug , Hash , Ord , PartialOrd)] pub struct Options { # [doc = " Define which driver to use by name if the `merge` attribute for a resource is unspecified."] # [doc = ""] # [doc = " This is the value of the `merge.default` git configuration."] pub default_driver : Option < BString > , }
};
}
