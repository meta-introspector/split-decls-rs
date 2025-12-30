// Generated macro for EncodedStringRef (enum)
macro_rules! Depcrate_snapshot_utilEncodedStringRef {
() => {
// Module: crate::snapshot::util
// Provides: {"EncodedStringRef"}
// Dependencies: {}
# [cfg_attr (test , derive (Debug))] # [derive (Clone , Copy)] pub enum EncodedStringRef < 'a > { Utf8 (& 'a str) , Unknown (& 'a BStr) , }
};
}
