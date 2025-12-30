// Generated macro for impl_436 (impl)
macro_rules! Depcrateimpl_436 {
() => {
// Module: crate
// Provides: {"impl_436"}
// Dependencies: {}
impl From < AssocItem > for ModuleDef { fn from (assoc : AssocItem) -> Self { match assoc { AssocItem :: Function (it) => ModuleDef :: Function (it) , AssocItem :: Const (it) => ModuleDef :: Const (it) , AssocItem :: TypeAlias (it) => ModuleDef :: TypeAlias (it) , } } }
};
}
