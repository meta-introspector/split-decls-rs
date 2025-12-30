// Generated macro for impl_18 (impl)
macro_rules! Depcrate_pkcs8impl_18 {
() => {
// Module: crate::pkcs8
// Provides: {"impl_18"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl EncodePrivateKey for KeypairBytes { fn to_pkcs8_der (& self) -> Result < SecretDocument > { let mut private_key = [0u8 ; 2 + (Self :: BYTE_SIZE / 2)] ; private_key [0] = 0x04 ; private_key [1] = 0x39 ; private_key [2 ..] . copy_from_slice (& self . secret_key) ; let private_key_info = PrivateKeyInfoRef { algorithm : ALGORITHM_ID , private_key : OctetStringRef :: new (& private_key) ? , public_key : self . public_key . as_ref () . map (| pk | BitStringRef :: new (0 , & pk . 0)) . transpose () ? , } ; let result = SecretDocument :: encode_msg (& private_key_info) ? ; # [cfg (feature = "zeroize")] private_key . zeroize () ; Ok (result) } }
};
}
