// Generated macro for ExtensionType (enum)
macro_rules! Depcrate_extensionsExtensionType {
() => {
// Module: crate::extensions
// Provides: {"ExtensionType"}
// Dependencies: {}
# [doc = " Defines the type of extension."] # [derive (Debug , PartialEq , Eq , Clone , Hash , PartialOrd , Ord , Copy)] # [non_exhaustive] pub enum ExtensionType { # [doc = " Transform Extension Type marked as `t`."] Transform , # [doc = " Unicode Extension Type marked as `u`."] Unicode , # [doc = " Private Extension Type marked as `x`."] Private , # [doc = " All other extension types."] Other (u8) , }
};
}
