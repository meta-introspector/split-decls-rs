// Generated macro for WriteHex (trait)
macro_rules! Depcrate_tests_parserWriteHex {
() => {
// Module: crate::tests::parser
// Provides: {"WriteHex"}
// Dependencies: {}
# [doc = "\nEncode a value as a hex string.\n\nImplementors of this trait should not write the `0x` prefix.\n"] pub trait WriteHex { # [doc = " Write the value as hex."] fn write_hex < W : fmt :: Write > (& self , writer : W) -> fmt :: Result ; }
};
}
