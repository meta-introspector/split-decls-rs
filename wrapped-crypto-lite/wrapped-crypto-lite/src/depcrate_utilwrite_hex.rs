// Generated macro for write_hex (function)
macro_rules! Depcrate_utilwrite_hex {
() => {
// Module: crate::util
// Provides: {"write_hex"}
// Dependencies: {}
# [doc = " Converts `bytes` to hexadecimal characters and writes them to `f`."] # [doc = " # Errors"] # [doc = " See [`core::fmt::write`]"] pub fn write_hex (f : & mut Formatter < '_ > , bytes : & [u8]) -> core :: fmt :: Result { for byte in bytes { write ! (f , "{byte:02X}") ? ; } Ok (()) }
};
}
