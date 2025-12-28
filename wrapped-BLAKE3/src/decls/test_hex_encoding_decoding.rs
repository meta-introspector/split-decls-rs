macro_rules! deps {
    () => {
        Hash!();
        Hasher!();
    };
}

macro_rules! test_hex_encoding_decoding {
    () => {
        deps!();
        # [test] fn test_hex_encoding_decoding () { let digest_str = "04e0bb39f30b1a3feb89f536c93be15055482df748674b00d26e5a75777702e9" ; let mut hasher = crate :: Hasher :: new () ; hasher . update (b"foo") ; let digest = hasher . finalize () ; assert_eq ! (digest . to_hex () . as_str () , digest_str) ; # [cfg (feature = "std")] assert_eq ! (digest . to_string () , digest_str) ; let digest = crate :: Hash :: from_hex (digest_str) . unwrap () ; assert_eq ! (digest . to_hex () . as_str () , digest_str) ; let digest = crate :: Hash :: from_hex (digest_str . to_uppercase ()) . unwrap () ; assert_eq ! (digest . to_hex () . as_str () , digest_str) ; let digest : crate :: Hash = digest_str . parse () . unwrap () ; assert_eq ! (digest . to_hex () . as_str () , digest_str) ; let bad_len = "04e0bb39f30b1" ; let _result = crate :: Hash :: from_hex (bad_len) . unwrap_err () ; # [cfg (feature = "std")] assert_eq ! (_result . to_string () , "expected 64 hex bytes, received 13") ; let bad_char = "Z4e0bb39f30b1a3feb89f536c93be15055482df748674b00d26e5a75777702e9" ; let _result = crate :: Hash :: from_hex (bad_char) . unwrap_err () ; # [cfg (feature = "std")] assert_eq ! (_result . to_string () , "invalid hex character: 'Z'") ; let _result = crate :: Hash :: from_hex ([128 ; 64]) . unwrap_err () ; # [cfg (feature = "std")] assert_eq ! (_result . to_string () , "invalid hex character: 0x80") ; }
    };
}

test_hex_encoding_decoding!();