// Generated macro for ZeroBlob (struct)
macro_rules! Depcrate_blobZeroBlob {
() => {
// Module: crate::blob
// Provides: {"ZeroBlob"}
// Dependencies: {}
# [doc = " BLOB of length N that is filled with zeroes."] # [doc = ""] # [doc = " Zeroblobs are intended to serve as placeholders for BLOBs whose content is"] # [doc = " later written using incremental BLOB I/O routines."] # [doc = ""] # [doc = " A negative value for the zeroblob results in a zero-length BLOB."] # [derive (Copy , Clone)] pub struct ZeroBlob (pub i32) ;
};
}
