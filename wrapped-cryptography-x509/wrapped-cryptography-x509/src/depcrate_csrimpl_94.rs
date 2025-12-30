// Generated macro for impl_94 (impl)
macro_rules! Depcrate_csrimpl_94 {
() => {
// Module: crate::csr
// Provides: {"impl_94"}
// Dependencies: {}
impl CertificationRequestInfo < '_ > { pub fn get_extension_attribute (& self ,) -> Result < Option < extensions :: RawExtensions < '_ > > , asn1 :: ParseError > { for attribute in self . attributes . unwrap_read () . clone () { if attribute . type_id == oid :: EXTENSION_REQUEST || attribute . type_id == oid :: MS_EXTENSION_REQUEST { check_attribute_length (attribute . values . unwrap_read () . clone ()) ? ; let val = attribute . values . unwrap_read () . clone () . next () . unwrap () ; let exts = asn1 :: parse_single (val . full_data ()) ? ; return Ok (Some (exts)) ; } } Ok (None) } }
};
}
