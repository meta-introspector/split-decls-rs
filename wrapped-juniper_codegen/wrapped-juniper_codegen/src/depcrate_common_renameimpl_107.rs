// Generated macro for impl_107 (impl)
macro_rules! Depcrate_common_renameimpl_107 {
() => {
// Module: crate::common::rename
// Provides: {"impl_107"}
// Dependencies: {}
impl TryFrom < syn :: LitStr > for Policy { type Error = syn :: Error ; fn try_from (lit : syn :: LitStr) -> syn :: Result < Self > { Self :: from_str (& lit . value ()) . map_err (| _ | syn :: Error :: new (lit . span () , "unknown renaming policy")) } }
};
}
