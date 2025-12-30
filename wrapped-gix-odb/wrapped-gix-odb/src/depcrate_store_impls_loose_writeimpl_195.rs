// Generated macro for impl_195 (impl)
macro_rules! Depcrate_store_impls_loose_writeimpl_195 {
() => {
// Module: crate::store_impls::loose::write
// Provides: {"impl_195"}
// Dependencies: {}
# [doc = " Access"] impl Store { # [doc = " Return the path to the object with `id`."] # [doc = ""] # [doc = " Note that is may not exist yet."] pub fn object_path (& self , id : & gix_hash :: oid) -> PathBuf { loose :: hash_path (id , self . path . clone ()) } }
};
}
