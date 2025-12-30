// Generated macro for ExtMeta (struct)
macro_rules! Depcrate_decode_extExtMeta {
() => {
// Module: crate::decode::ext
// Provides: {"ExtMeta"}
// Dependencies: {}
# [doc = " Extension type meta information."] # [doc = ""] # [doc = " Extension represents a tuple of type information and a byte array where type information is an"] # [doc = " integer whose meaning is defined by applications."] # [doc = ""] # [doc = " Applications can assign 0 to 127 to store application-specific type information."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " MessagePack reserves -1 to -128 for future extension to add predefined types which will be"] # [doc = " described in separated documents."] # [derive (Debug , PartialEq)] pub struct ExtMeta { # [doc = " Type information."] pub typeid : i8 , # [doc = " Byte array size."] pub size : u32 , }
};
}
