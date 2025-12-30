// Generated macro for fn_kind (function)
macro_rules! Depcrate_delegationfn_kind {
() => {
// Module: crate::delegation
// Provides: {"fn_kind"}
// Dependencies: {}
fn fn_kind < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId) -> FnKind { debug_assert_matches ! (tcx . def_kind (def_id) , DefKind :: Fn | DefKind :: AssocFn) ; let parent = tcx . parent (def_id) ; match tcx . def_kind (parent) { DefKind :: Trait => FnKind :: AssocTrait , DefKind :: Impl { of_trait : true } => FnKind :: AssocTraitImpl , DefKind :: Impl { of_trait : false } => FnKind :: AssocInherentImpl , _ => FnKind :: Free , } }
};
}
