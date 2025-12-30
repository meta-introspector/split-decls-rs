// Generated macro for seal_dst_out_length (function)
macro_rules! Depcrate_test_framework_aead_interfaceseal_dst_out_length {
() => {
// Module: crate::test_framework::aead_interface
// Provides: {"seal_dst_out_length"}
// Dependencies: {}
# [cfg (feature = "safe_api")] # [doc = " Related bug: <https://github.com/orion-rs/orion/issues/52>"] # [doc = " Test dst_out mutable array sizes when using seal()."] fn seal_dst_out_length < Sealer , Key , Nonce > (sealer : & Sealer , key : & Key , nonce : & Nonce , input : & [u8] , tag_size : usize , aad : & [u8] ,) where Sealer : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , { let default_aad = if aad . is_empty () { None } else { Some (aad) } ; let mut dst_out_ct = vec ! [0u8 ; input . len () + tag_size] ; assert ! (sealer (key , nonce , input , default_aad , & mut dst_out_ct) . is_ok ()) ; let mut dst_out_ct_more = vec ! [0u8 ; input . len () + (tag_size + 1)] ; assert ! (sealer (key , nonce , input , default_aad , & mut dst_out_ct_more) . is_ok ()) ; let mut dst_out_ct_more_double = vec ! [0u8 ; input . len () + (tag_size * 2)] ; assert ! (sealer (key , nonce , input , default_aad , & mut dst_out_ct_more_double) . is_ok ()) ; let mut dst_out_ct_less = vec ! [0u8 ; input . len () + (tag_size - 1)] ; assert ! (sealer (key , nonce , input , default_aad , & mut dst_out_ct_less) . is_err ()) ; }
};
}
