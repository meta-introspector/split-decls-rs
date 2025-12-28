macro_rules! get_owner_return_paths {
    () => {
        # [doc = " Given a `DefId` for an opaque type in return position, find its parent item's return"] # [doc = " expressions."] fn get_owner_return_paths (tcx : TyCtxt < '_ > , def_id : LocalDefId ,) -> Option < (LocalDefId , ReturnsVisitor < '_ >) > { let hir_id = tcx . local_def_id_to_hir_id (def_id) ; let parent_id = tcx . hir_get_parent_item (hir_id) . def_id ; tcx . hir_node_by_def_id (parent_id) . body_id () . map (| body_id | { let body = tcx . hir_body (body_id) ; let mut visitor = ReturnsVisitor :: default () ; visitor . visit_body (body) ; (parent_id , visitor) }) }
    };
}

get_owner_return_paths!();