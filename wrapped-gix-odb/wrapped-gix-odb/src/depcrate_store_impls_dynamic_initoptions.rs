// Generated macro for Options (struct)
macro_rules! Depcrate_store_impls_dynamic_initOptions {
() => {
// Module: crate::store_impls::dynamic::init
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for use in [`Store::at_opts()`]."] # [derive (Clone , Debug)] pub struct Options { # [doc = " How to obtain a size for the slot map."] pub slots : Slots , # [doc = " The kind of hash we expect in our packs and would use for loose object iteration and object writing."] pub object_hash : gix_hash :: Kind , # [doc = " If false, no multi-pack indices will be used. If true, they will be used if their hash matches `object_hash`."] pub use_multi_pack_index : bool , # [doc = " The current directory of the process at the time of instantiation."] # [doc = " If unset, it will be retrieved using `gix_fs::current_dir(false)`."] pub current_dir : Option < std :: path :: PathBuf > , }
};
}
