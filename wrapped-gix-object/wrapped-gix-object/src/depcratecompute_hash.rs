// Generated macro for compute_hash (function)
macro_rules! Depcratecompute_hash {
() => {
// Module: crate
// Provides: {"compute_hash"}
// Dependencies: {}
# [doc = " A function to compute a hash of kind `hash_kind` for an object of `object_kind` and its `data`."] # [doc (alias = "hash_object" , alias = "git2")] pub fn compute_hash (hash_kind : gix_hash :: Kind , object_kind : Kind , data : & [u8] ,) -> Result < gix_hash :: ObjectId , gix_hash :: hasher :: Error > { let mut hasher = object_hasher (hash_kind , object_kind , data . len () as u64) ; hasher . update (data) ; hasher . try_finalize () }
};
}
