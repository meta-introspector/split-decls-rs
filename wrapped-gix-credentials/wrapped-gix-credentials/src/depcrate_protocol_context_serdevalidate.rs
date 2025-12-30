// Generated macro for validate (function)
macro_rules! Depcrate_protocol_context_serdevalidate {
() => {
// Module: crate::protocol::context::serde
// Provides: {"validate"}
// Dependencies: {}
fn validate (key : & str , value : & BStr) -> Result < () , Error > { if key . contains ('\0') || key . contains ('\n') || value . contains (& 0) || value . contains (& b'\n') { return Err (Error :: Encoding { key : key . to_owned () , value : value . to_owned () , }) ; } Ok (()) }
};
}
