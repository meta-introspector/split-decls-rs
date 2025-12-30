// Generated macro for NSZone (struct)
macro_rules! Depcrate_runtime_nszoneNSZone {
() => {
// Module: crate::runtime::nszone
// Provides: {"NSZone"}
// Dependencies: {}
# [doc = " A type used to identify and manage memory zones."] # [doc = ""] # [doc = " Zones are ignored on all newer platforms, you should very rarely need to"] # [doc = " use this, but may be useful if you need to implement `copyWithZone:` or"] # [doc = " `allocWithZone:`."] # [doc = ""] # [doc = " See [Apple's documentation](https://developer.apple.com/documentation/foundation/nszone?language=objc)."] # [repr (C)] pub struct NSZone { _priv : [u8 ; 0] , _inner : ffi :: OpaqueData , }
};
}
