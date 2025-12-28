macro_rules! deps {
    () => {
        ItemCtxt!();
        FeedConstTy!();
    };
}

macro_rules! const_param_default {
    () => {
        deps!();
        fn const_param_default < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId ,) -> ty :: EarlyBinder < 'tcx , Const < 'tcx > > { let default_ct = match tcx . hir_node_by_def_id (def_id) { hir :: Node :: GenericParam (hir :: GenericParam { kind : hir :: GenericParamKind :: Const { default : Some (ct) , .. } , .. }) => ct , _ => span_bug ! (tcx . def_span (def_id) , "`const_param_default` expected a generic parameter with a constant") , } ; let icx = ItemCtxt :: new (tcx , def_id) ; let identity_args = ty :: GenericArgs :: identity_for_item (tcx , def_id) ; let ct = icx . lowerer () . lower_const_arg (default_ct , FeedConstTy :: Param (def_id . to_def_id () , identity_args)) ; ty :: EarlyBinder :: bind (ct) }
    };
}

const_param_default!();