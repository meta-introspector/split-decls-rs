// Generated macro for impl_394 (impl)
macro_rules! Depcrate_scriptimpl_394 {
() => {
// Module: crate::script
// Provides: {"impl_394"}
// Dependencies: {}
impl AsULE for ScriptWithExt { type ULE = < u16 as AsULE > :: ULE ; # [inline] fn to_unaligned (self) -> Self :: ULE { Script (self . 0) . to_unaligned () } # [inline] fn from_unaligned (unaligned : Self :: ULE) -> Self { ScriptWithExt (Script :: from_unaligned (unaligned) . 0) } }
};
}
