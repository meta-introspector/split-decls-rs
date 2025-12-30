// Generated macro for none_or_empty_some_aad_same_result (function)
macro_rules! Depcrate_test_framework_aead_interfacenone_or_empty_some_aad_same_result {
() => {
// Module: crate::test_framework::aead_interface
// Provides: {"none_or_empty_some_aad_same_result"}
// Dependencies: {}
# [cfg (feature = "safe_api")] # [doc = " Using None or Some with empty slice should produce the exact same result."] fn none_or_empty_some_aad_same_result < Sealer , Opener , Key , Nonce > (sealer : & Sealer , opener : & Opener , key : & Key , nonce : & Nonce , input : & [u8] , tag_size : usize ,) where Sealer : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , Opener : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , { let mut dst_out_ct_none = vec ! [0u8 ; input . len () + tag_size] ; let mut dst_out_ct_some_empty = vec ! [0u8 ; input . len () + tag_size] ; sealer (key , nonce , input , None , & mut dst_out_ct_none) . unwrap () ; sealer (key , nonce , input , Some (& [0u8 ; 0]) , & mut dst_out_ct_some_empty ,) . unwrap () ; assert_eq ! (dst_out_ct_none , dst_out_ct_some_empty) ; let mut dst_out_pt = vec ! [0u8 ; input . len ()] ; assert ! (opener (key , nonce , & dst_out_ct_none , Some (& [0u8 ; 0]) , & mut dst_out_pt) . is_ok ()) ; assert ! (opener (key , nonce , & dst_out_ct_some_empty , None , & mut dst_out_pt) . is_ok ()) ; }
};
}
