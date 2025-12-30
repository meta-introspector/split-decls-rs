// Generated macro for encode (function)
macro_rules! Depcrateencode {
() => {
// Module: crate
// Provides: {"encode"}
// Dependencies: {}
pub (crate) fn encode < 'a > (encoding_override : EncodingOverride < '_ > , input : & 'a str) -> Cow < 'a , [u8] > { if let Some (o) = encoding_override { return o (input) ; } input . as_bytes () . into () }
};
}
