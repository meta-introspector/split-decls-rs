// Generated macro for ResetStream (struct)
macro_rules! Depcrate_frameResetStream {
() => {
// Module: crate::frame
// Provides: {"ResetStream"}
// Dependencies: {}
# [allow (unreachable_pub)] # [cfg_attr (feature = "arbitrary" , derive (Arbitrary))] # [derive (Debug , Copy , Clone)] pub struct ResetStream { pub (crate) id : StreamId , pub (crate) error_code : VarInt , pub (crate) final_offset : VarInt , }
};
}
