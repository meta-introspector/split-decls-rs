// Generated macro for AccessEvent (enum)
macro_rules! Depcrate_shims_native_libAccessEvent {
() => {
// Module: crate::shims::native_lib
// Provides: {"AccessEvent"}
// Dependencies: {}
# [doc = " A single memory access."] # [derive (Serialize , Deserialize , Clone , Debug)] pub enum AccessEvent { # [doc = " A read occurred on this memory range."] Read (AccessRange) , # [doc = " A write may have occurred on this memory range."] # [doc = " Some instructions *may* write memory without *always* doing that,"] # [doc = " so this can be an over-approximation."] # [doc = " The range info, however, is reliable if the access did happen."] # [doc = " If the second field is true, the access definitely happened."] Write (AccessRange , bool) , }
};
}
