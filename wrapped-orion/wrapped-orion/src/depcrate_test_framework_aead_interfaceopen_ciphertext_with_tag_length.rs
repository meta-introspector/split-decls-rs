// Generated macro for open_ciphertext_with_tag_length (function)
macro_rules! Depcrate_test_framework_aead_interfaceopen_ciphertext_with_tag_length {
() => {
// Module: crate::test_framework::aead_interface
// Provides: {"open_ciphertext_with_tag_length"}
// Dependencies: {}
# [cfg (feature = "safe_api")] # [doc = " Test input sizes when using open()."] fn open_ciphertext_with_tag_length < Sealer , Opener , Key , Nonce > (sealer : & Sealer , opener : & Opener , key : & Key , nonce : & Nonce , tag_size : usize , aad : & [u8] ,) where Sealer : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , Opener : Fn (& Key , & Nonce , & [u8] , Option < & [u8] > , & mut [u8]) -> Result < () , UnknownCryptoError > , { let default_aad = if aad . is_empty () { None } else { Some (aad) } ; let mut dst_out_pt = vec ! [0u8 ; tag_size] ; assert ! (opener (key , nonce , & [0u8 ; 0] , default_aad , & mut dst_out_pt) . is_err ()) ; assert ! (opener (key , nonce , & vec ! [0u8 ; tag_size - 1] , default_aad , & mut dst_out_pt) . is_err ()) ; let mut dst_out_ct = vec ! [0u8 ; tag_size] ; sealer (key , nonce , & [0u8 ; 0] , default_aad , & mut dst_out_ct) . unwrap () ; assert ! (opener (key , nonce , & dst_out_ct , default_aad , & mut dst_out_pt) . is_ok ()) ; }
};
}
