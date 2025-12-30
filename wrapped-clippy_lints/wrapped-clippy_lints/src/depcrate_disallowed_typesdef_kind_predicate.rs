// Generated macro for def_kind_predicate (function)
macro_rules! Depcrate_disallowed_typesdef_kind_predicate {
() => {
// Module: crate::disallowed_types
// Provides: {"def_kind_predicate"}
// Dependencies: {}
pub fn def_kind_predicate (def_kind : DefKind) -> bool { matches ! (def_kind , DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Trait | DefKind :: TyAlias | DefKind :: ForeignTy | DefKind :: AssocTy) }
};
}
