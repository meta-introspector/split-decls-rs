// Generated macro for impl_23 (impl)
macro_rules! Depcrate_encoding_boximpl_23 {
() => {
// Module: crate::encoding_box
// Provides: {"impl_23"}
// Dependencies: {}
impl EncodingBox { # [doc = " Same as [`Encoding::C_LONG`]."] pub const C_LONG : Self = match Encoding :: C_LONG { Encoding :: Long => Self :: Long , Encoding :: LongLong => Self :: LongLong , _ => unreachable ! () , } ; # [doc = " Same as [`Encoding::C_ULONG`]."] pub const C_ULONG : Self = match Encoding :: C_ULONG { Encoding :: ULong => Self :: ULong , Encoding :: ULongLong => Self :: ULongLong , _ => unreachable ! () , } ; # [doc = " Parse and consume an encoding from the start of a string."] # [doc = ""] # [doc = " This is can be used to parse concatenated encodings, such as those"] # [doc = " returned by `method_getTypeEncoding`."] # [doc = ""] # [doc = " [`from_str`][Self::from_str] is simpler, use that instead if you can."] # [doc = ""] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns an error if the string was an ill-formatted encoding string."] pub fn from_start_of_str (s : & mut & str) -> Result < Self , ParseError > { let mut parser = Parser :: new (s) ; parser . strip_leading_qualifiers () ; match parser . parse_encoding_or_none () { Err (ErrorKind :: Unknown (b'0' ..= b'9')) => { let remaining = parser . remaining () ; * s = remaining ; Ok (EncodingBox :: None) } Err (err) => Err (ParseError :: new (parser , err)) , Ok (encoding) => { let remaining = parser . remaining () ; * s = remaining ; Ok (encoding) } } } }
};
}
