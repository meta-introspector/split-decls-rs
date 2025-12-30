// Generated macro for write_hex_bytes (function)
macro_rules! Depcrate_debugwrite_hex_bytes {
() => {
// Module: crate::debug
// Provides: {"write_hex_bytes"}
// Dependencies: {}
pub (crate) fn write_hex_bytes (fmt : & mut core :: fmt :: Formatter , bytes : & [u8] ,) -> Result < () , core :: fmt :: Error > { for byte in bytes { write ! (fmt , "{byte:02x}") ? ; } Ok (()) }
};
}
