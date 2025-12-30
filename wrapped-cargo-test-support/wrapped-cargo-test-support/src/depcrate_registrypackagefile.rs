// Generated macro for PackageFile (struct)
macro_rules! Depcrate_registryPackageFile {
() => {
// Module: crate::registry
// Provides: {"PackageFile"}
// Dependencies: {}
# [doc = " A file to be created in a package."] struct PackageFile { path : String , contents : EntryData , # [doc = " The Unix mode for the file. Note that when extracted on Windows, this"] # [doc = " is mostly ignored since it doesn't have the same style of permissions."] mode : u32 , # [doc = " If `true`, the file is created in the root of the tarfile, used for"] # [doc = " testing invalid packages."] extra : bool , }
};
}
