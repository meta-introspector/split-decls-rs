// Generated macro for impl_408 (impl)
macro_rules! Depcrateimpl_408 {
() => {
// Module: crate
// Provides: {"impl_408"}
// Dependencies: {}
impl From < ModuleDef > for ItemInNs { fn from (module_def : ModuleDef) -> Self { match module_def { ModuleDef :: Static (_) | ModuleDef :: Const (_) | ModuleDef :: Function (_) => { ItemInNs :: Values (module_def) } ModuleDef :: Macro (it) => ItemInNs :: Macros (it) , _ => ItemInNs :: Types (module_def) , } } }
};
}
