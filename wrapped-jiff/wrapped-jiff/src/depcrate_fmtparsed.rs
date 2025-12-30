// Generated macro for Parsed (struct)
macro_rules! Depcrate_fmtParsed {
() => {
// Module: crate::fmt
// Provides: {"Parsed"}
// Dependencies: {}
# [doc = " The result of parsing a value out of a slice of bytes."] # [doc = ""] # [doc = " This contains both the parsed value and the offset at which the value"] # [doc = " ended in the input given. This makes it possible to parse, for example, a"] # [doc = " datetime value as a prefix of some larger string without knowing ahead of"] # [doc = " time where it ends."] # [derive (Clone)] pub (crate) struct Parsed < 'i , V > { # [doc = " The value parsed."] value : V , # [doc = " The remaining unparsed input."] input : & 'i [u8] , }
};
}
