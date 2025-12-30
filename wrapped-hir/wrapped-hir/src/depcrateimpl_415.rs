// Generated macro for impl_415 (impl)
macro_rules! Depcrateimpl_415 {
() => {
// Module: crate
// Provides: {"impl_415"}
// Dependencies: {}
impl From < ModuleDef > for ItemInNs { fn from (module_def : ModuleDef) -> Self { match module_def { ModuleDef :: Static (_) | ModuleDef :: Const (_) | ModuleDef :: Function (_) => { ItemInNs :: Values (module_def) } ModuleDef :: Macro (it) => ItemInNs :: Macros (it) , _ => ItemInNs :: Types (module_def) , } } }
};
}
