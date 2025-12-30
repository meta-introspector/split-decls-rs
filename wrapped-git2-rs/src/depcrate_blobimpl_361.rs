// Generated macro for impl_361 (impl)
macro_rules! Depcrate_blobimpl_361 {
() => {
// Module: crate::blob
// Provides: {"impl_361"}
// Dependencies: {}
impl < 'repo > BlobWriter < 'repo > { # [doc = " Finalize blob writing stream and write the blob to the object db"] pub fn commit (mut self) -> Result < Oid , Error > { self . need_cleanup = false ; let mut raw = raw :: git_oid { id : [0 ; raw :: GIT_OID_RAWSZ] , } ; unsafe { try_call ! (raw :: git_blob_create_fromstream_commit (& mut raw , self . raw)) ; Ok (Binding :: from_raw (& raw as * const _)) } } }
};
}
