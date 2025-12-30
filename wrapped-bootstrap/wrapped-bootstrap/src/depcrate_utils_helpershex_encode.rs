// Generated macro for hex_encode (function)
macro_rules! Depcrate_utils_helpershex_encode {
() => {
// Module: crate::utils::helpers
// Provides: {"hex_encode"}
// Dependencies: {}
# [doc = " Converts `T` into a hexadecimal `String`."] pub fn hex_encode < T > (input : T) -> String where T : AsRef < [u8] > , { use std :: fmt :: Write ; input . as_ref () . iter () . fold (String :: with_capacity (input . as_ref () . len () * 2) , | mut acc , & byte | { write ! (& mut acc , "{byte:02x}") . expect ("Failed to write byte to the hex String.") ; acc }) }
};
}
