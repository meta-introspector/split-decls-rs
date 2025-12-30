// Generated macro for seal_open_equals_expected (function)
macro_rules! Depcrate_test_framework_aead_interfaceseal_open_equals_expected {
() => {
// Module: crate::test_framework::aead_interface
// Provides: {"seal_open_equals_expected"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] # [cfg (feature = "safe_api")] # [doc = " Test that sealing and opening produces the expected ciphertext."] fn seal_open_equals_expected < Sealer , Opener , Key , Nonce > (sealer : & Sealer , opener : & Opener , key : & Key , nonce : & Nonce , input : & [u8] , expected_ct_with_tag : Option < & [u8] > , tag_size : usize , aad : & [u8] ,) where Sealer : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , Opener : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , { let default_aad = if aad . is_empty () { None } else { Some (aad) } ; let mut dst_out_ct = vec ! [0u8 ; input . len () + tag_size] ; sealer (key , nonce , input , default_aad , & mut dst_out_ct) . unwrap () ; if let Some (expected) = expected_ct_with_tag { assert_eq ! (expected , & dst_out_ct [..]) ; } let mut dst_out_pt = input . to_vec () ; opener (key , nonce , & dst_out_ct , default_aad , & mut dst_out_pt) . unwrap () ; assert_eq ! (input , & dst_out_pt [..]) ; if let Some (expected) = expected_ct_with_tag { opener (key , nonce , expected , default_aad , & mut dst_out_pt) . unwrap () ; assert_eq ! (input , & dst_out_pt [..]) ; } }
};
}
