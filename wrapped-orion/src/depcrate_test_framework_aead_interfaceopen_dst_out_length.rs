// Generated macro for open_dst_out_length (function)
macro_rules! Depcrate_test_framework_aead_interfaceopen_dst_out_length {
() => {
// Module: crate::test_framework::aead_interface
// Provides: {"open_dst_out_length"}
// Dependencies: {}
# [cfg (feature = "safe_api")] # [doc = " Related bug: <https://github.com/orion-rs/orion/issues/52>"] # [doc = " Test dst_out mutable array sizes when using open()."] fn open_dst_out_length < Sealer , Opener , Key , Nonce > (sealer : & Sealer , opener : & Opener , key : & Key , nonce : & Nonce , input : & [u8] , tag_size : usize , aad : & [u8] ,) where Sealer : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , Opener : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , { let default_aad = if aad . is_empty () { None } else { Some (aad) } ; let mut dst_out_ct = vec ! [0u8 ; input . len () + tag_size] ; sealer (key , nonce , input , default_aad , & mut dst_out_ct) . unwrap () ; let mut dst_out_pt = vec ! [0u8 ; input . len ()] ; assert ! (opener (key , nonce , & dst_out_ct , default_aad , & mut dst_out_pt) . is_ok ()) ; let mut dst_out_pt_0 = [0u8 ; 0] ; let empty_out_res = opener (key , nonce , & dst_out_ct , default_aad , & mut dst_out_pt_0) ; if input . is_empty () { assert ! (empty_out_res . is_ok ()) ; } else { assert ! (empty_out_res . is_err ()) ; } if ! input . is_empty () { let mut dst_out_pt_less = vec ! [0u8 ; input . len () - 1] ; assert ! (opener (key , nonce , & dst_out_ct , default_aad , & mut dst_out_pt_less) . is_err ()) ; } let mut dst_out_pt_more = vec ! [0u8 ; input . len () + 1] ; assert ! (opener (key , nonce , & dst_out_ct , default_aad , & mut dst_out_pt_more) . is_ok ()) ; }
};
}
