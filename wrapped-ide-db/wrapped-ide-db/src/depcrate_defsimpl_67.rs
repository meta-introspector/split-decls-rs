// Generated macro for impl_67 (impl)
macro_rules! Depcrate_defsimpl_67 {
() => {
// Module: crate::defs
// Provides: {"impl_67"}
// Dependencies: {}
impl From < ModuleDef > for Definition { fn from (def : ModuleDef) -> Self { match def { ModuleDef :: Module (it) => Definition :: Module (it) , ModuleDef :: Function (it) => Definition :: Function (it) , ModuleDef :: Adt (it) => Definition :: Adt (it) , ModuleDef :: Variant (it) => Definition :: Variant (it) , ModuleDef :: Const (it) => Definition :: Const (it) , ModuleDef :: Static (it) => Definition :: Static (it) , ModuleDef :: Trait (it) => Definition :: Trait (it) , ModuleDef :: TypeAlias (it) => Definition :: TypeAlias (it) , ModuleDef :: Macro (it) => Definition :: Macro (it) , ModuleDef :: BuiltinType (it) => Definition :: BuiltinType (it) , } } }
};
}
