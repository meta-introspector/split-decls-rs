// Generated macro for tests (module)
macro_rules! Depcrate_base64tests {
() => {
// Module: crate::base64
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_encode_block () { assert_eq ! ("" . to_string () , encode_block (b"")) ; assert_eq ! ("Zg==" . to_string () , encode_block (b"f")) ; assert_eq ! ("Zm8=" . to_string () , encode_block (b"fo")) ; assert_eq ! ("Zm9v" . to_string () , encode_block (b"foo")) ; assert_eq ! ("Zm9vYg==" . to_string () , encode_block (b"foob")) ; assert_eq ! ("Zm9vYmE=" . to_string () , encode_block (b"fooba")) ; assert_eq ! ("Zm9vYmFy" . to_string () , encode_block (b"foobar")) ; } # [test] fn test_decode_block () { assert_eq ! (b"" . to_vec () , decode_block ("") . unwrap ()) ; assert_eq ! (b"f" . to_vec () , decode_block ("Zg==") . unwrap ()) ; assert_eq ! (b"fo" . to_vec () , decode_block ("Zm8=") . unwrap ()) ; assert_eq ! (b"foo" . to_vec () , decode_block ("Zm9v") . unwrap ()) ; assert_eq ! (b"foob" . to_vec () , decode_block ("Zm9vYg==") . unwrap ()) ; assert_eq ! (b"fooba" . to_vec () , decode_block ("Zm9vYmE=") . unwrap ()) ; assert_eq ! (b"foobar" . to_vec () , decode_block ("Zm9vYmFy") . unwrap ()) ; } # [test] fn test_strip_whitespace () { assert_eq ! (b"foobar" . to_vec () , decode_block (" Zm9vYmFy\n") . unwrap ()) ; assert_eq ! (b"foob" . to_vec () , decode_block (" Zm9vYg==\n") . unwrap ()) ; } }
};
}
