// Generated macro for impl_32 (impl)
macro_rules! Depcrate_pbes1impl_32 {
() => {
// Module: crate::pbes1
// Provides: {"impl_32"}
// Dependencies: {}
impl TryFrom < ObjectIdentifier > for EncryptionScheme { type Error = der :: Error ; fn try_from (oid : ObjectIdentifier) -> der :: Result < Self > { match oid { PBE_WITH_MD2_AND_DES_CBC_OID => Ok (Self :: PbeWithMd2AndDesCbc) , PBE_WITH_MD2_AND_RC2_CBC_OID => Ok (Self :: PbeWithMd2AndRc2Cbc) , PBE_WITH_MD5_AND_DES_CBC_OID => Ok (Self :: PbeWithMd5AndDesCbc) , PBE_WITH_MD5_AND_RC2_CBC_OID => Ok (Self :: PbeWithMd5AndRc2Cbc) , PBE_WITH_SHA1_AND_DES_CBC_OID => Ok (Self :: PbeWithSha1AndDesCbc) , PBE_WITH_SHA1_AND_RC2_CBC_OID => Ok (Self :: PbeWithSha1AndRc2Cbc) , _ => Err (ErrorKind :: OidUnknown { oid } . into ()) , } } }
};
}
