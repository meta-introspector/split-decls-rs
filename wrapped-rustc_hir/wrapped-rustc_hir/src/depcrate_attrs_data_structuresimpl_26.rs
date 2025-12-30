// Generated macro for impl_26 (impl)
macro_rules! Depcrate_attrs_data_structuresimpl_26 {
() => {
// Module: crate::attrs::data_structures
// Provides: {"impl_26"}
// Dependencies: {}
impl InlineAttr { pub fn always (& self) -> bool { match self { InlineAttr :: Always | InlineAttr :: Force { .. } => true , InlineAttr :: None | InlineAttr :: Hint | InlineAttr :: Never => false , } } }
};
}
