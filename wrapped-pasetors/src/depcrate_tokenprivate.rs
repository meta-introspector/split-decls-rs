// Generated macro for private (module)
macro_rules! Depcrate_tokenprivate {
() => {
// Module: crate::token
// Provides: {"private"}
// Dependencies: {}
pub (crate) mod private { use super :: Error ; use crate :: version :: private :: Version ; # [doc = " Purpose (`local`/`public`) of a token, given a version `V`."] pub trait Purpose < V : Version > { # [doc = " Validate the header for a given version and purpose for some token."] fn validate_header (token : & str) -> Result < () , Error > ; # [doc = " Validate the tokens raw (decoded base64)"] # [doc = " message length for a given version and purpose for some token."] fn validate_token_message_len (message : & [u8]) -> Result < () , Error > ; # [doc = " Parse the raw payload of a token. Either the ciphertext or the message that was signed."] # [doc = " The length **MUST** have been verified beforehand."] fn parse_raw_payload (message : & [u8]) -> & [u8] ; } }
};
}
