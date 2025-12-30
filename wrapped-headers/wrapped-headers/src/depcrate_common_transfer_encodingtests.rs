// Generated macro for tests (module)
macro_rules! Depcrate_common_transfer_encodingtests {
() => {
// Module: crate::common::transfer_encoding
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: super :: test_decode ; use super :: TransferEncoding ; # [test] fn chunked_is_chunked () { assert ! (TransferEncoding :: chunked () . is_chunked ()) ; } # [test] fn decode_gzip_chunked_is_chunked () { let te = test_decode :: < TransferEncoding > (& ["gzip, chunked"]) . unwrap () ; assert ! (te . is_chunked ()) ; } # [test] fn decode_chunked_gzip_is_not_chunked () { let te = test_decode :: < TransferEncoding > (& ["chunked, gzip"]) . unwrap () ; assert ! (! te . is_chunked ()) ; } # [test] fn decode_notchunked_is_not_chunked () { let te = test_decode :: < TransferEncoding > (& ["notchunked"]) . unwrap () ; assert ! (! te . is_chunked ()) ; } # [test] fn decode_multiple_is_chunked () { let te = test_decode :: < TransferEncoding > (& ["gzip" , "chunked"]) . unwrap () ; assert ! (te . is_chunked ()) ; } }
};
}
