// Generated macro for parse_signature (function)
macro_rules! Depcrate_parseparse_signature {
() => {
// Module: crate::parse
// Provides: {"parse_signature"}
// Dependencies: {}
pub (crate) fn parse_signature (raw : & BStr) -> Result < gix_actor :: SignatureRef < '_ > , crate :: decode :: Error > { gix_actor :: SignatureRef :: from_bytes :: < crate :: decode :: ParseError > (raw . as_ref ()) . map_err (| err | crate :: decode :: Error :: with_err (err , raw . as_ref ())) }
};
}
