// Generated macro for impl_233 (impl)
macro_rules! Depcrate_tokenimpl_233 {
() => {
// Module: crate::token
// Provides: {"impl_233"}
// Dependencies: {}
# [cfg (feature = "std")] impl TryFrom < & TrustedToken > for Footer { type Error = Error ; fn try_from (value : & TrustedToken) -> Result < Self , Self :: Error > { if value . footer . is_empty () { return Err (Error :: FooterParsing) ; } let mut footer = Footer :: new () ; footer . parse_bytes (value . footer ()) ? ; Ok (footer) } }
};
}
