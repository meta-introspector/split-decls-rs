// Generated macro for def_to_ty (function)
macro_rules! Depcrate_searchdef_to_ty {
() => {
// Module: crate::search
// Provides: {"def_to_ty"}
// Dependencies: {}
fn def_to_ty < 'db > (sema : & Semantics < 'db , RootDatabase > , def : & Definition) -> Option < hir :: Type < 'db > > { match def { Definition :: Adt (adt) => Some (adt . ty (sema . db)) , Definition :: TypeAlias (it) => Some (it . ty (sema . db)) , Definition :: BuiltinType (it) => Some (it . ty (sema . db)) , Definition :: SelfType (it) => Some (it . self_ty (sema . db)) , _ => None , } }
};
}
