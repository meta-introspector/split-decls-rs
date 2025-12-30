// Generated macro for modify_base (function)
macro_rules! Depcrate_index_writemodify_base {
() => {
// Module: crate::index::write
// Provides: {"modify_base"}
// Dependencies: {}
fn modify_base (entry : & mut TreeEntry , pack_entry : & crate :: data :: Entry , decompressed : & [u8] , hash : gix_hash :: Kind ,) -> Result < () , gix_hash :: hasher :: Error > { let object_kind = pack_entry . header . as_kind () . expect ("base object as source of iteration") ; let id = gix_object :: compute_hash (hash , object_kind , decompressed) ? ; entry . id = id ; Ok (()) }
};
}
