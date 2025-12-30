// Generated macro for impl_229 (impl)
macro_rules! Depcrate_tokenimpl_229 {
() => {
// Module: crate::token
// Provides: {"impl_229"}
// Dependencies: {}
impl < V : Version > Purpose < V > for Public { fn validate_header (token : & str) -> Result < () , Error > { if token . is_empty () || ! token . starts_with (V :: PUBLIC_HEADER) { return Err (Error :: TokenFormat) ; } Ok (()) } fn validate_token_message_len (message : & [u8]) -> Result < () , Error > { if message . len () <= V :: PUBLIC_SIG { return Err (Error :: TokenFormat) ; } Ok (()) } fn parse_raw_payload (message : & [u8]) -> & [u8] { debug_assert ! (message . len () > V :: PUBLIC_SIG) ; & message [.. message . len () - V :: PUBLIC_SIG] } }
};
}
