// Generated macro for impl_22 (impl)
macro_rules! Depcrate_object_idimpl_22 {
() => {
// Module: crate::object_id
// Provides: {"impl_22"}
// Dependencies: {}
# [doc = " Sha1 hash specific methods"] impl ObjectId { # [doc = " Instantiate an Digest from 20 bytes of a Sha1 digest."] # [inline] fn new_sha1 (id : [u8 ; SIZE_OF_SHA1_DIGEST]) -> Self { ObjectId :: Sha1 (id) } # [doc = " Instantiate an Digest from a slice 20 borrowed bytes of a Sha1 digest."] # [doc = ""] # [doc = " Panics of the slice doesn't have a length of 20."] # [inline] pub (crate) fn from_20_bytes (b : & [u8]) -> ObjectId { let mut id = [0 ; SIZE_OF_SHA1_DIGEST] ; id . copy_from_slice (b) ; ObjectId :: Sha1 (id) } # [doc = " Returns an Digest representing a Sha1 with whose memory is zeroed."] # [inline] pub (crate) const fn null_sha1 () -> ObjectId { ObjectId :: Sha1 ([0u8 ; 20]) } }
};
}
