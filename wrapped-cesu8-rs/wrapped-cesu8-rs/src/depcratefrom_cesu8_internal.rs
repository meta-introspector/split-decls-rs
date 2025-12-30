// Generated macro for from_cesu8_internal (function)
macro_rules! Depcratefrom_cesu8_internal {
() => {
// Module: crate
// Provides: {"from_cesu8_internal"}
// Dependencies: {}
# [doc = " Do the actual work of decoding."] fn from_cesu8_internal (bytes : & [u8] , variant : Variant) -> Result < Cow < str > , Cesu8DecodingError > { match from_utf8 (bytes) { Ok (str) => Ok (Cow :: Borrowed (str)) , _ => { let mut decoded = Vec :: with_capacity (bytes . len ()) ; if decode_from_iter (& mut decoded , & mut bytes . iter () , variant) { debug_assert ! (from_utf8 (& decoded [..]) . is_ok ()) ; Ok (Cow :: Owned (unsafe { String :: from_utf8_unchecked (decoded) })) } else { Err (Cesu8DecodingError) } } } }
};
}
