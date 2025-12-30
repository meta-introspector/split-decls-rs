// Generated macro for tests (module)
macro_rules! Depcrate_lziptests {
() => {
// Module: crate::lzip
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_decode_dict_size () { let dict_size = decode_dict_size (0xD3) . unwrap () ; assert_eq ! (dict_size , 320 * 1024) ; let dict_size = decode_dict_size (0x0C) . unwrap () ; assert_eq ! (dict_size , 4 * 1024) ; let dict_size = decode_dict_size (0x1D) . unwrap () ; assert_eq ! (dict_size , 512 * 1024 * 1024) ; assert ! (decode_dict_size (0x0B) . is_err ()) ; assert ! (decode_dict_size (0x1E) . is_err ()) ; } # [test] fn test_encode_dict_size () { assert_eq ! (encode_dict_size (4 * 1024) . unwrap () , 0x0C) ; assert_eq ! (encode_dict_size (512 * 1024 * 1024) . unwrap () , 0x1D) ; assert_eq ! (encode_dict_size (320 * 1024) . unwrap () , 0xD3) ; assert ! (encode_dict_size (1024) . is_err ()) ; assert ! (encode_dict_size (1024 * 1024 * 1024) . is_err ()) ; } }
};
}
