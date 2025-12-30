// Generated macro for decode (function)
macro_rules! Depcrate_extension_tree_decodedecode {
() => {
// Module: crate::extension::tree::decode
// Provides: {"decode"}
// Dependencies: {}
# [doc = " A recursive data structure"] pub fn decode (data : & [u8] , object_hash : gix_hash :: Kind) -> Option < Tree > { let (tree , data) = one_recursive (data , object_hash . len_in_bytes ()) ? ; assert ! (data . is_empty () , "BUG: should fully consume the entire tree extension chunk, got {} left" , data . len ()) ; Some (tree) }
};
}
