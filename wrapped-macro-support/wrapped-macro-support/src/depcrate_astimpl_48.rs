// Generated macro for impl_48 (impl)
macro_rules! Depcrate_astimpl_48 {
() => {
// Module: crate::ast
// Provides: {"impl_48"}
// Dependencies: {}
impl ImportKind { # [doc = " Whether this type can be inside an `impl` block."] pub fn fits_on_impl (& self) -> bool { match * self { ImportKind :: Function (_) => true , ImportKind :: Static (_) => false , ImportKind :: String (_) => false , ImportKind :: Type (_) => false , ImportKind :: Enum (_) => false , } } }
};
}
