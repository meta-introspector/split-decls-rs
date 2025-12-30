// Generated macro for impl_175 (impl)
macro_rules! Depcrate_store_impls_loose_iterimpl_175 {
() => {
// Module: crate::store_impls::loose::iter
// Provides: {"impl_175"}
// Dependencies: {}
impl loose :: Iter { fn path_to_id (& self , res : Result < fs :: walkdir :: DirEntry , fs :: walkdir :: Error > ,) -> Option < Result < gix_hash :: ObjectId , Error > > { use std :: path :: Component :: Normal ; match res { Ok (e) => { let p = e . path () ; let mut ci = p . components () ; let (c2 , c1) = (ci . next_back () , ci . next_back ()) ; if let (Some (Normal (c1)) , Some (Normal (c2))) = (c1 , c2) { if c1 . len () == 2 && c2 . len () == self . hash_hex_len - 2 { if let (Some (c1) , Some (c2)) = (c1 . to_str () , c2 . to_str ()) { let mut buf = gix_hash :: Kind :: hex_buf () ; { let (first_byte , rest) = buf [.. self . hash_hex_len] . split_at_mut (2) ; first_byte . copy_from_slice (c1 . as_bytes ()) ; rest . copy_from_slice (c2 . as_bytes ()) ; } if let Ok (b) = gix_hash :: ObjectId :: from_hex (& buf [.. self . hash_hex_len]) { return Some (Ok (b)) ; } } } } } Err (err) => return Some (Err (err)) , } None } }
};
}
