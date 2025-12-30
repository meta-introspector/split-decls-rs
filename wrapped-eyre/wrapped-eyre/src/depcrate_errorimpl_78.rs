// Generated macro for impl_78 (impl)
macro_rules! Depcrate_errorimpl_78 {
() => {
// Module: crate::error
// Provides: {"impl_78"}
// Dependencies: {}
impl From < Report > for Box < dyn StdError + 'static > { fn from (error : Report) -> Self { Box :: < dyn StdError + Send + Sync > :: from (error) } }
};
}
