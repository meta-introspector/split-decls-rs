// Generated macro for tests (module)
macro_rules! Depcrate_hkdftests {
() => {
// Module: crate::hkdf
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [allow (clippy :: expect_used , clippy :: panic , clippy :: indexing_slicing , clippy :: unwrap_used)] mod tests { use crate :: { digest :: Sha256 , hkdf :: { HkdfSha256 , HkdfSha512 , Prk , Salt } , test_helpers :: { decode_hex , decode_hex_into_vec } , } ; # [test] fn sha256 () { let ikm = decode_hex_into_vec ("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b") ; let salt_vec = decode_hex_into_vec ("000102030405060708090a0b0c") ; let salt = Salt :: NonEmpty (& salt_vec) ; let info = decode_hex_into_vec ("f0f1f2f3f4f5f6f7f8f9") ; let okm : [u8 ; 42] = HkdfSha256 :: derive (ikm . as_slice () , salt , info . as_slice ()) ; let expected = decode_hex ("3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865" ,) ; assert_eq ! (okm , expected) ; } # [test] fn sha512 () { let ikm = decode_hex_into_vec ("5d3db20e8238a90b62a600fa57fdb318") ; let salt_vec = decode_hex_into_vec ("1d6f3b38a1e607b5e6bcd4af1800a9d3") ; let salt = Salt :: NonEmpty (& salt_vec) ; let info = decode_hex_into_vec ("2bc5f39032b6fc87da69ba8711ce735b169646fd") ; let okm : [u8 ; 42] = HkdfSha512 :: derive (ikm . as_slice () , salt , info . as_slice ()) ; let expected = decode_hex ("8c3cf7122dcb5eb7efaf02718f1faf70bca20dcb75070e9d0871a413a6c05fc195a75aa9ffc349d70aae" ,) ; assert_eq ! (okm , expected) ; } # [test] fn rfc5869_sha256 () { struct Test { ikm : Vec < u8 > , salt : Vec < u8 > , info : Vec < u8 > , prk : Vec < u8 > , okm : Vec < u8 > , } let tests = [Test { ikm : decode_hex_into_vec ("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b") , salt : decode_hex_into_vec ("000102030405060708090a0b0c") , info : decode_hex_into_vec ("f0f1f2f3f4f5f6f7f8f9") , prk : decode_hex_into_vec ("077709362c2e32df0ddc3f0dc47bba63\
                    90b6c73bb50f9c3122ec844ad7c2b3e5" ,) , okm : decode_hex_into_vec ("3cb25f25faacd57a90434f64d0362f2a2d2d0a90cf1a5a4c5db02d56ecc4c5bf34007208d5b887185865") } , Test { ikm : decode_hex_into_vec ("000102030405060708090a0b0c0d0e0f\
                    101112131415161718191a1b1c1d1e1f\
                    202122232425262728292a2b2c2d2e2f\
                    303132333435363738393a3b3c3d3e3f\
                    404142434445464748494a4b4c4d4e4f" ,) , salt : decode_hex_into_vec ("606162636465666768696a6b6c6d6e6f\
                    707172737475767778797a7b7c7d7e7f\
                    808182838485868788898a8b8c8d8e8f\
                    909192939495969798999a9b9c9d9e9f\
                    a0a1a2a3a4a5a6a7a8a9aaabacadaeaf" ,) , info : decode_hex_into_vec ("b0b1b2b3b4b5b6b7b8b9babbbcbdbebf\
                    c0c1c2c3c4c5c6c7c8c9cacbcccdcecf\
                    d0d1d2d3d4d5d6d7d8d9dadbdcdddedf\
                    e0e1e2e3e4e5e6e7e8e9eaebecedeeef\
                    f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff" ,) , prk : decode_hex_into_vec ("06a6b88c5853361a06104c9ceb35b45c\
                    ef760014904671014a193f40c15fc244" ,) , okm : decode_hex_into_vec ("b11e398dc80327a1c8e7f78c596a4934\
                    4f012eda2d4efad8a050cc4c19afa97c\
                    59045a99cac7827271cb41c65e590e09\
                    da3275600c2f09b8367793a9aca3db71\
                    cc30c58179ec3e87c14c01d5c1f3434f\
                    1d87" ,) } , Test { ikm : decode_hex_into_vec ("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b") , salt : Vec :: new () , info : Vec :: new () , prk : decode_hex_into_vec ("19ef24a32c717b167f33a91d6f648bdf\
                    96596776afdb6377ac434c1c293ccb04" ,) , okm : decode_hex_into_vec ("8da4e775a563c18f715f802a063c5a31b8a11f5c5ee1879ec3454e5f3c738d2d9d201395faa4b61a96c8") , } ,] ; for Test { ikm , salt , info , prk , okm , } in tests . iter () { let salt = if salt . is_empty () { Salt :: None } else { Salt :: NonEmpty (& salt) } ; let mut okm2 = vec ! [0u8 ; okm . len ()] ; assert ! (HkdfSha256 :: derive_into (ikm . as_slice () , salt , info . as_slice () , & mut okm2) . is_ok ()) ; assert_eq ! (okm2 . as_slice () , okm . as_slice ()) ; let prk2 = Prk :: new :: < Sha256 > (prk . as_slice ()) . unwrap () ; assert_eq ! (prk2 . as_bytes () , prk . as_slice ()) ; let mut okm3 = vec ! [0u8 ; okm . len ()] ; let _ = prk2 . expand_into (info . as_slice () , & mut okm3) ; assert_eq ! (okm3 . as_slice () , okm . as_slice ()) ; } } # [test] fn max_output () { let hkdf = HkdfSha256 :: extract (b"" , Salt :: None) ; let mut longest = vec ! [0u8 ; HkdfSha256 :: MAX_OUTPUT_LEN] ; assert ! (hkdf . expand_into (b"" , & mut longest) . is_ok ()) ; let mut too_long = vec ! [0u8 ; HkdfSha256 :: MAX_OUTPUT_LEN + 1] ; assert ! (hkdf . expand_into (b"" , & mut too_long) . is_err ()) ; } # [test] fn wrong_prk_len () { assert ! (Prk :: new ::< Sha256 > (decode_hex_into_vec ("077709362c2e32df0ddc3f0dc47bba63") . as_slice ()) . is_none ()) ; assert ! (Prk :: new ::< Sha256 > (decode_hex_into_vec ("077709362c2e32df0ddc3f0dc47bba6390b6c73bb50f9c3122ec844ad7c2b3e590b6c73bb50f9c3122ec844ad7c2b3e5") . as_slice ()) . is_none ()) ; } }
};
}
