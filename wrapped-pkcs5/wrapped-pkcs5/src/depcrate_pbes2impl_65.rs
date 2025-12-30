// Generated macro for impl_65 (impl)
macro_rules! Depcrate_pbes2impl_65 {
() => {
// Module: crate::pbes2
// Provides: {"impl_65"}
// Dependencies: {}
impl TryFrom < AlgorithmIdentifierRef < '_ > > for EncryptionScheme { type Error = der :: Error ; fn try_from (alg : AlgorithmIdentifierRef < '_ >) -> der :: Result < Self > { let iv = match alg . parameters { Some (params) => params . decode_as :: < & OctetStringRef > () ? . as_bytes () , None => return Err (Tag :: OctetString . value_error () . into ()) , } ; match alg . oid { AES_128_CBC_OID => Ok (Self :: Aes128Cbc { iv : iv . try_into () . map_err (| _ | Tag :: OctetString . value_error ()) ? , }) , AES_192_CBC_OID => Ok (Self :: Aes192Cbc { iv : iv . try_into () . map_err (| _ | Tag :: OctetString . value_error ()) ? , }) , AES_256_CBC_OID => Ok (Self :: Aes256Cbc { iv : iv . try_into () . map_err (| _ | Tag :: OctetString . value_error ()) ? , }) , AES_128_GCM_OID => Ok (Self :: Aes128Gcm { nonce : iv . try_into () . map_err (| _ | Tag :: OctetString . value_error ()) ? , }) , AES_256_GCM_OID => Ok (Self :: Aes256Gcm { nonce : iv . try_into () . map_err (| _ | Tag :: OctetString . value_error ()) ? , }) , # [cfg (feature = "des-insecure")] DES_CBC_OID => Ok (Self :: DesCbc { iv : iv [0 .. DES_BLOCK_SIZE] . try_into () . map_err (| _ | Tag :: OctetString . value_error ()) ? , }) , # [cfg (feature = "3des")] DES_EDE3_CBC_OID => Ok (Self :: DesEde3Cbc { iv : iv [0 .. DES_BLOCK_SIZE] . try_into () . map_err (| _ | Tag :: OctetString . value_error ()) ? , }) , oid => Err (ErrorKind :: OidUnknown { oid } . into ()) , } } }
};
}
