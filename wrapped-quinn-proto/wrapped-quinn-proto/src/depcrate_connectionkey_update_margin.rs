// Generated macro for KEY_UPDATE_MARGIN (const)
macro_rules! Depcrate_connectionKEY_UPDATE_MARGIN {
() => {
// Module: crate::connection
// Provides: {"KEY_UPDATE_MARGIN"}
// Dependencies: {}
# [doc = " Perform key updates this many packets before the AEAD confidentiality limit."] # [doc = ""] # [doc = " Chosen arbitrarily, intended to be large enough to prevent spurious connection loss."] const KEY_UPDATE_MARGIN : u64 = 10_000 ;
};
}
