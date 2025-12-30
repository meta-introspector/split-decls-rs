// Generated macro for version_is_supported (function)
macro_rules! Depcrateversion_is_supported {
() => {
// Module: crate
// Provides: {"version_is_supported"}
// Dependencies: {}
# [doc = " Returns true if the given protocol version is supported."] # [inline] pub fn version_is_supported (version : u32) -> bool { matches ! (version , PROTOCOL_VERSION_V1) }
};
}
