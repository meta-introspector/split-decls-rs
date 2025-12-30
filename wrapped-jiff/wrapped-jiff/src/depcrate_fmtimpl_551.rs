// Generated macro for impl_551 (impl)
macro_rules! Depcrate_fmtimpl_551 {
() => {
// Module: crate::fmt
// Provides: {"impl_551"}
// Dependencies: {}
impl < 'i , V : core :: fmt :: Display > Parsed < 'i , V > { # [doc = " Ensures that the parsed value represents the entire input. This occurs"] # [doc = " precisely when the `input` on this parsed value is empty."] # [doc = ""] # [doc = " This is useful when one expects a parsed value to consume the entire"] # [doc = " input, and to consider it an error if it doesn't."] # [inline] fn into_full (self) -> Result < V , Error > { if self . input . is_empty () { return Ok (self . value) ; } Err (err ! ("parsed value '{value}', but unparsed input {unparsed:?} \
             remains (expected no unparsed input)" , value = self . value , unparsed = escape :: Bytes (self . input) ,)) } }
};
}
