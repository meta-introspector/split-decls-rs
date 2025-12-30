// Generated macro for macro_121 (macro)
macro_rules! Depcrate_asn1macro_121 {
() => {
// Module: crate::asn1
// Provides: {"macro_121"}
// Dependencies: {}
foreign_type_and_impl_send_sync ! { type CType = ffi :: ASN1_OBJECT ; fn drop = ffi :: ASN1_OBJECT_free ; fn clone = ffi :: OBJ_dup ; # [doc = " Object Identifier"] # [doc = ""] # [doc = " Represents an ASN.1 Object.  Typically, NIDs, or numeric identifiers"] # [doc = " are stored as a table within the [`Nid`] module.  These constants are"] # [doc = " used to determine attributes of a certificate, such as mapping the"] # [doc = " attribute \"CommonName\" to \"CN\" which is represented as the OID of 13."] # [doc = " This attribute is a constant in the [`nid::COMMONNAME`]."] # [doc = ""] # [doc = " OpenSSL documentation at [`OBJ_nid2obj`]"] # [doc = ""] # [doc = " [`Nid`]: ../nid/index.html"] # [doc = " [`nid::COMMONNAME`]: ../nid/constant.COMMONNAME.html"] # [doc = " [`OBJ_nid2obj`]: https://docs.openssl.org/master/man3/OBJ_obj2nid/"] pub struct Asn1Object ; # [doc = " A reference to an [`Asn1Object`]."] pub struct Asn1ObjectRef ; }
};
}
