// Generated macro for local_item_child_by_name (function)
macro_rules! Depcrate_pathslocal_item_child_by_name {
() => {
// Module: crate::paths
// Provides: {"local_item_child_by_name"}
// Dependencies: {}
fn local_item_child_by_name (tcx : TyCtxt < '_ > , local_id : LocalDefId , ns : PathNS , name : Symbol) -> Option < DefId > { let root_mod ; let item_kind = match tcx . hir_node_by_def_id (local_id) { Node :: Crate (r#mod) => { root_mod = ItemKind :: Mod (Ident :: dummy () , r#mod) ; & root_mod } , Node :: Item (item) => & item . kind , _ => return None , } ; match item_kind { ItemKind :: Mod (_ , r#mod) => r#mod . item_ids . iter () . find_map (| & item_id | { let item = tcx . hir_item (item_id) ; if let ItemKind :: Use (path , UseKind :: Single (ident)) = item . kind { if ident . name == name { let opt_def_id = | ns : Option < Res > | ns . and_then (| res | res . opt_def_id ()) ; match ns { PathNS :: Type => opt_def_id (path . res . type_ns) , PathNS :: Value => opt_def_id (path . res . value_ns) , PathNS :: Macro => opt_def_id (path . res . macro_ns) , PathNS :: Arbitrary => unreachable ! () , } } else { None } } else if let Some (ident) = item . kind . ident () && ident . name == name && ns . matches (tcx . def_kind (item . owner_id) . ns ()) { Some (item . owner_id . to_def_id ()) } else { None } }) , ItemKind :: Impl (..) | ItemKind :: Trait (..) => tcx . associated_items (local_id) . filter_by_name_unhygienic (name) . find (| assoc_item | ns . matches (Some (assoc_item . namespace ()))) . map (| assoc_item | assoc_item . def_id) , _ => None , } }
};
}
