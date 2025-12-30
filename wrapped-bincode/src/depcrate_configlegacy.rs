// Generated macro for legacy (function)
macro_rules! Depcrate_configlegacy {
() => {
// Module: crate::config
// Provides: {"legacy"}
// Dependencies: {}
# [doc = " Creates the \"legacy\" default config. This is the default config that was present in bincode 1.0"] # [doc = " - Little endian"] # [doc = " - Fixed int length encoding"] pub const fn legacy () -> Configuration < LittleEndian , Fixint , NoLimit > { generate () }
};
}
