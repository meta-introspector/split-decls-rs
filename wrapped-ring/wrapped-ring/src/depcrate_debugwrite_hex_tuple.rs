// Generated macro for write_hex_tuple (function)
macro_rules! Depcrate_debugwrite_hex_tuple {
() => {
// Module: crate::debug
// Provides: {"write_hex_tuple"}
// Dependencies: {}
pub (crate) fn write_hex_tuple (fmt : & mut core :: fmt :: Formatter , type_name : & str , value : & dyn AsRef < [u8] > ,) -> Result < () , :: core :: fmt :: Error > { fmt . debug_tuple (type_name) . field (& HexStr (value . as_ref ())) . finish () }
};
}
