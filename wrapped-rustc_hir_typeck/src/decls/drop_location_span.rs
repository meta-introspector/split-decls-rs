macro_rules! deps {
    () => {
        ItemKind!();
    };
}

macro_rules! drop_location_span {
    () => {
        deps!();
        # [doc = " Returns the Span of where the value with the provided HirId would be dropped"] fn drop_location_span (tcx : TyCtxt < '_ > , hir_id : HirId) -> Span { let owner_id = tcx . hir_get_enclosing_scope (hir_id) . unwrap () ; let owner_node = tcx . hir_node (owner_id) ; let owner_span = match owner_node { hir :: Node :: Item (item) => match item . kind { hir :: ItemKind :: Fn { body : owner_id , .. } => tcx . hir_span (owner_id . hir_id) , _ => { bug ! ("Drop location span error: need to handle more ItemKind '{:?}'" , item . kind) ; } } , hir :: Node :: Block (block) => tcx . hir_span (block . hir_id) , hir :: Node :: TraitItem (item) => tcx . hir_span (item . hir_id ()) , hir :: Node :: ImplItem (item) => tcx . hir_span (item . hir_id ()) , _ => { bug ! ("Drop location span error: need to handle more Node '{:?}'" , owner_node) ; } } ; tcx . sess . source_map () . end_point (owner_span) }
    };
}

drop_location_span!();