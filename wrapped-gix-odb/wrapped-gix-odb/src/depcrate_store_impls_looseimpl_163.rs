// Generated macro for impl_163 (impl)
macro_rules! Depcrate_store_impls_looseimpl_163 {
() => {
// Module: crate::store_impls::loose
// Provides: {"impl_163"}
// Dependencies: {}
# [doc = " Initialization"] impl Store { # [doc = " Initialize the Db with the `objects_directory` containing the hexadecimal first byte subdirectories, which in turn"] # [doc = " contain all loose objects."] # [doc = ""] # [doc = " In a git repository, this would be `.git/objects`."] # [doc = ""] # [doc = " The `object_hash` determines which hash to use when writing, finding or iterating objects."] pub fn at (objects_directory : impl Into < PathBuf > , object_hash : gix_hash :: Kind) -> Store { Store { path : objects_directory . into () , object_hash , } } # [doc = " Return the path to our `objects` directory."] pub fn path (& self) -> & Path { & self . path } # [doc = " Return the kind of hash we would iterate and write."] pub fn object_hash (& self) -> gix_hash :: Kind { self . object_hash } }
};
}
