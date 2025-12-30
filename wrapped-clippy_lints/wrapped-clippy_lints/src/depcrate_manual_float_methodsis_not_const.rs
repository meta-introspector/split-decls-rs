// Generated macro for is_not_const (function)
macro_rules! Depcrate_manual_float_methodsis_not_const {
() => {
// Module: crate::manual_float_methods
// Provides: {"is_not_const"}
// Dependencies: {}
fn is_not_const (tcx : TyCtxt < '_ > , def_id : DefId) -> bool { match tcx . def_kind (def_id) { DefKind :: Mod | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Variant | DefKind :: Trait | DefKind :: TyAlias | DefKind :: ForeignTy | DefKind :: TraitAlias | DefKind :: AssocTy | DefKind :: Macro (..) | DefKind :: Field | DefKind :: LifetimeParam | DefKind :: ExternCrate | DefKind :: Use | DefKind :: ForeignMod | DefKind :: GlobalAsm | DefKind :: Impl { .. } | DefKind :: OpaqueTy | DefKind :: SyntheticCoroutineBody | DefKind :: TyParam => true , DefKind :: AnonConst | DefKind :: InlineConst | DefKind :: Const | DefKind :: ConstParam | DefKind :: Static { .. } | DefKind :: Ctor (..) | DefKind :: AssocConst => false , DefKind :: Fn | DefKind :: AssocFn | DefKind :: Closure => tcx . constness (def_id) == Constness :: NotConst , } }
};
}
