// Generated macro for non_local_item_child_by_name (function)
macro_rules! Depcrate_pathsnon_local_item_child_by_name {
() => {
// Module: crate::paths
// Provides: {"non_local_item_child_by_name"}
// Dependencies: {}
fn non_local_item_child_by_name (tcx : TyCtxt < '_ > , def_id : DefId , ns : PathNS , name : Symbol) -> Option < DefId > { match tcx . def_kind (def_id) { DefKind :: Mod | DefKind :: Enum | DefKind :: Trait => tcx . module_children (def_id) . iter () . find_map (| child | { if child . ident . name == name && ns . matches (child . res . ns ()) { child . res . opt_def_id () } else { None } }) , DefKind :: Impl { .. } => tcx . associated_item_def_ids (def_id) . iter () . copied () . find (| assoc_def_id | tcx . item_name (* assoc_def_id) == name && ns . matches (tcx . def_kind (assoc_def_id) . ns ())) , _ => None , } }
};
}
