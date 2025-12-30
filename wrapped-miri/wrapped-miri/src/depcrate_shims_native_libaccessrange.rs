// Generated macro for AccessRange (struct)
macro_rules! Depcrate_shims_native_libAccessRange {
() => {
// Module: crate::shims::native_lib
// Provides: {"AccessRange"}
// Dependencies: {}
# [doc = " The memory touched by a given access."] # [derive (Serialize , Deserialize , Clone , Debug)] pub struct AccessRange { # [doc = " The base address in memory where an access occurred."] pub addr : usize , # [doc = " The number of bytes affected from the base."] pub size : usize , }
};
}
