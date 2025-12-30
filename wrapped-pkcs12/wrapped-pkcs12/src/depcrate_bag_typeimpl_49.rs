// Generated macro for impl_49 (impl)
macro_rules! Depcrate_bag_typeimpl_49 {
() => {
// Module: crate::bag_type
// Provides: {"impl_49"}
// Dependencies: {}
impl From < BagType > for ObjectIdentifier { fn from (content_type : BagType) -> ObjectIdentifier { match content_type { BagType :: Key => crate :: PKCS_12_KEY_BAG_OID , BagType :: Pkcs8 => crate :: PKCS_12_PKCS8_KEY_BAG_OID , BagType :: Cert => crate :: PKCS_12_CERT_BAG_OID , BagType :: Crl => crate :: PKCS_12_CRL_BAG_OID , BagType :: Secret => crate :: PKCS_12_SECRET_BAG_OID , BagType :: SafeContents => crate :: PKCS_12_SAFE_CONTENTS_BAG_OID , } } }
};
}
