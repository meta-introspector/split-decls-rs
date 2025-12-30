// Generated macro for impl_500 (impl)
macro_rules! Depcrateimpl_500 {
() => {
// Module: crate
// Provides: {"impl_500"}
// Dependencies: {}
impl From < ItemInNs > for ScopeDef { fn from (item : ItemInNs) -> Self { match item { ItemInNs :: Types (id) => ScopeDef :: ModuleDef (id) , ItemInNs :: Values (id) => ScopeDef :: ModuleDef (id) , ItemInNs :: Macros (id) => ScopeDef :: ModuleDef (ModuleDef :: Macro (id)) , } } }
};
}
