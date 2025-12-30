// Generated macro for test (module)
macro_rules! Depcrate_zlib_bufreadtest {
() => {
// Module: crate::zlib::bufread
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use crate :: bufread :: ZlibDecoder ; use crate :: zlib :: write ; use crate :: Compression ; use std :: io :: { Read , Write } ; # [test] fn decode_extra_data () { let expected = "Hello World" ; let compressed = { let mut e = write :: ZlibEncoder :: new (Vec :: new () , Compression :: default ()) ; e . write_all (expected . as_ref ()) . unwrap () ; let mut b = e . finish () . unwrap () ; b . push (b'x') ; b } ; let mut output = Vec :: new () ; let mut decoder = ZlibDecoder :: new (compressed . as_slice ()) ; let decoded_bytes = decoder . read_to_end (& mut output) . unwrap () ; assert_eq ! (decoded_bytes , output . len ()) ; let actual = std :: str :: from_utf8 (& output) . expect ("String parsing error") ; assert_eq ! (actual , expected , "after decompression we obtain the original input") ; output . clear () ; assert_eq ! (decoder . read (& mut output) . unwrap () , 0 , "subsequent read of decoder returns 0, but inner reader can return additional data") ; let mut reader = decoder . into_inner () ; assert_eq ! (reader . read_to_end (& mut output) . unwrap () , 1 , "extra data is accessible in underlying buf-read") ; assert_eq ! (output , b"x") ; } }
};
}
