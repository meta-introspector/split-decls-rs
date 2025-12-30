// Generated macro for EncodingOverride (type)
macro_rules! DepcrateEncodingOverride {
() => {
// Module: crate
// Provides: {"EncodingOverride"}
// Dependencies: {}
pub type EncodingOverride < 'a > = Option < & 'a dyn Fn (& str) -> Cow < '_ , [u8] > > ;
};
}
