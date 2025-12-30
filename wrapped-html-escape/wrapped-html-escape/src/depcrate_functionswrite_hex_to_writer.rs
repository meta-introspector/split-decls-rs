// Generated macro for write_hex_to_writer (function)
macro_rules! Depcrate_functionswrite_hex_to_writer {
() => {
// Module: crate::functions
// Provides: {"write_hex_to_writer"}
// Dependencies: {}
# [cfg (feature = "std")] # [inline] pub (crate) fn write_hex_to_writer < W : Write > (e : u8 , output : & mut W) -> Result < () , io :: Error > { output . write_fmt (format_args ! ("&#x{e:02X};")) }
};
}
