// Generated macro for impl_552 (impl)
macro_rules! Depcrate_fmtimpl_552 {
() => {
// Module: crate::fmt
// Provides: {"impl_552"}
// Dependencies: {}
impl < 'i , V > Parsed < 'i , V > { # [doc = " Ensures that the parsed value represents the entire input. This occurs"] # [doc = " precisely when the `input` on this parsed value is empty."] # [doc = ""] # [doc = " This is useful when one expects a parsed value to consume the entire"] # [doc = " input, and to consider it an error if it doesn't."] # [doc = ""] # [doc = " This is like `Parsed::into_full`, but lets the caller provide a custom"] # [doc = " `Display` implementation."] # [inline] fn into_full_with (self , display : impl core :: fmt :: Display ,) -> Result < V , Error > { if self . input . is_empty () { return Ok (self . value) ; } Err (err ! ("parsed value '{value}', but unparsed input {unparsed:?} \
             remains (expected no unparsed input)" , value = display , unparsed = escape :: Bytes (self . input) ,)) } }
};
}
