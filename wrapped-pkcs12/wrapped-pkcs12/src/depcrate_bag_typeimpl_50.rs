// Generated macro for impl_50 (impl)
macro_rules! Depcrate_bag_typeimpl_50 {
() => {
// Module: crate::bag_type
// Provides: {"impl_50"}
// Dependencies: {}
impl TryFrom < ObjectIdentifier > for BagType { type Error = der :: Error ; fn try_from (oid : ObjectIdentifier) -> der :: Result < Self > { match oid { crate :: PKCS_12_KEY_BAG_OID => Ok (Self :: Key) , crate :: PKCS_12_PKCS8_KEY_BAG_OID => Ok (Self :: Pkcs8) , crate :: PKCS_12_CERT_BAG_OID => Ok (Self :: Cert) , crate :: PKCS_12_CRL_BAG_OID => Ok (Self :: Crl) , crate :: PKCS_12_SECRET_BAG_OID => Ok (Self :: Secret) , crate :: PKCS_12_SAFE_CONTENTS_BAG_OID => Ok (Self :: SafeContents) , _ => Err (ErrorKind :: OidUnknown { oid } . into ()) , } } }
};
}
