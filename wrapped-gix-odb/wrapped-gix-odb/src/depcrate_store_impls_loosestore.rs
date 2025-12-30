// Generated macro for Store (struct)
macro_rules! Depcrate_store_impls_looseStore {
() => {
// Module: crate::store_impls::loose
// Provides: {"Store"}
// Dependencies: {}
# [doc = " A database for reading and writing objects to disk, one file per object."] # [derive (Clone , PartialEq , Eq)] pub struct Store { # [doc = " The directory in which objects are stored, containing 256 folders representing the hashes first byte."] pub (crate) path : PathBuf , # [doc = " The kind of hash we should assume during iteration and when writing new objects."] pub (crate) object_hash : gix_hash :: Kind , }
};
}
