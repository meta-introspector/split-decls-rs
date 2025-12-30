// Generated macro for strip_preamble (function)
macro_rules! Depcrate_grammarstrip_preamble {
() => {
// Module: crate::grammar
// Provides: {"strip_preamble"}
// Dependencies: {}
# [doc = " Strip the \"preamble\", i.e. data that appears before the PEM"] # [doc = " pre-encapsulation boundary."] # [doc = ""] # [doc = " Presently no attempt is made to ensure the preamble decodes successfully"] # [doc = " under any particular character encoding. The only byte which is disallowed"] # [doc = " is the NUL byte. This restriction does not appear in RFC7468, but rather"] # [doc = " is inspired by the OpenSSL PEM decoder."] # [doc = ""] # [doc = " Returns a slice which starts at the beginning of the encapsulated text."] # [doc = ""] # [doc = " From RFC7468:"] # [doc = " > Data before the encapsulation boundaries are permitted, and"] # [doc = " > parsers MUST NOT malfunction when processing such data."] pub (crate) fn strip_preamble (mut bytes : & [u8]) -> Result < & [u8] > { if bytes . starts_with (PRE_ENCAPSULATION_BOUNDARY) { return Ok (bytes) ; } while let Some ((byte , remaining)) = bytes . split_first () { match * byte { CHAR_NUL => { return Err (Error :: Preamble) ; } CHAR_LF if remaining . starts_with (PRE_ENCAPSULATION_BOUNDARY) => { return Ok (remaining) ; } _ => () , } bytes = remaining ; } Err (Error :: Preamble) }
};
}
