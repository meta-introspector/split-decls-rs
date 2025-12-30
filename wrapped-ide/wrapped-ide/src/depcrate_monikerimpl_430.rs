// Generated macro for impl_430 (impl)
macro_rules! Depcrate_monikerimpl_430 {
() => {
// Module: crate::moniker
// Provides: {"impl_430"}
// Dependencies: {}
impl From < SymbolInformationKind > for MonikerDescriptorKind { fn from (value : SymbolInformationKind) -> Self { match value { SymbolInformationKind :: AssociatedType => Self :: Type , SymbolInformationKind :: Attribute => Self :: Meta , SymbolInformationKind :: Constant => Self :: Term , SymbolInformationKind :: Enum => Self :: Type , SymbolInformationKind :: EnumMember => Self :: Type , SymbolInformationKind :: Field => Self :: Term , SymbolInformationKind :: Function => Self :: Method , SymbolInformationKind :: Macro => Self :: Macro , SymbolInformationKind :: Method => Self :: Method , SymbolInformationKind :: Module => Self :: Namespace , SymbolInformationKind :: Parameter => Self :: Parameter , SymbolInformationKind :: SelfParameter => Self :: Parameter , SymbolInformationKind :: StaticMethod => Self :: Method , SymbolInformationKind :: StaticVariable => Self :: Term , SymbolInformationKind :: Struct => Self :: Type , SymbolInformationKind :: Trait => Self :: Type , SymbolInformationKind :: TraitMethod => Self :: Method , SymbolInformationKind :: Type => Self :: Type , SymbolInformationKind :: TypeAlias => Self :: Type , SymbolInformationKind :: TypeParameter => Self :: TypeParameter , SymbolInformationKind :: Union => Self :: Type , SymbolInformationKind :: Variable => Self :: Term , } } }
};
}
