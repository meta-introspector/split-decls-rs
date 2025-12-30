// Generated macro for with_test_item_names (function)
macro_rules! Depcratewith_test_item_names {
() => {
// Module: crate
// Provides: {"with_test_item_names"}
// Dependencies: {}
# [doc = " Apply `f()` to the set of test item names."] # [doc = " The names are sorted using the default `Symbol` ordering."] fn with_test_item_names (tcx : TyCtxt < '_ > , module : LocalModDefId , f : impl FnOnce (& [Symbol]) -> bool) -> bool { let cache = TEST_ITEM_NAMES_CACHE . get_or_init (| | Mutex :: new (FxHashMap :: default ())) ; let mut map : MutexGuard < '_ , FxHashMap < LocalModDefId , Vec < Symbol > > > = cache . lock () . unwrap () ; let value = map . entry (module) ; match value { Entry :: Occupied (entry) => f (entry . get ()) , Entry :: Vacant (entry) => { let mut names = Vec :: new () ; for id in tcx . hir_module_free_items (module) { if matches ! (tcx . def_kind (id . owner_id) , DefKind :: Const) && let item = tcx . hir_item (id) && let ItemKind :: Const (ident , _generics , ty , _body) = item . kind && let TyKind :: Path (QPath :: Resolved (_ , path)) = ty . kind && let Res :: Def (DefKind :: Struct , _) = path . res { let has_test_marker = tcx . hir_attrs (item . hir_id ()) . iter () . any (| a | a . has_name (sym :: rustc_test_marker)) ; if has_test_marker { names . push (ident . name) ; } } } names . sort_unstable () ; f (entry . insert (names)) } , } }
};
}
