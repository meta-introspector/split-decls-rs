// Generated macro for Tag (struct)
macro_rules! Depcrate_cmacTag {
() => {
// Module: crate::cmac
// Provides: {"Tag"}
// Dependencies: {}
# [doc = " A CMAC tag."] # [doc = ""] # [doc = " For a given tag `t`, use `t.as_ref()` to get the tag value as a byte slice."] # [derive (Clone , Copy , Debug)] pub struct Tag { bytes : [u8 ; MAX_CMAC_TAG_LEN] , len : usize , }
};
}
