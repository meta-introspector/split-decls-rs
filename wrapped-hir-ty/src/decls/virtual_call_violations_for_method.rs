macro_rules! deps {
    () => {
        MethodViolationCode!();
        GenericPredicates!();
        HirDatabase!();
        AllowSelfProjection!();
    };
}

macro_rules! virtual_call_violations_for_method {
    () => {
        deps!();
        fn virtual_call_violations_for_method < F > (db : & dyn HirDatabase , trait_ : TraitId , func : FunctionId , cb : & mut F ,) -> ControlFlow < () > where F : FnMut (MethodViolationCode) -> ControlFlow < () > , { let func_data = db . function_signature (func) ; if ! func_data . has_self_param () { cb (MethodViolationCode :: StaticMethod) ? ; } if func_data . is_async () { cb (MethodViolationCode :: AsyncFn) ? ; } let sig = db . callable_item_signature (func . into ()) ; if sig . skip_binder () . inputs () . iter () . skip (1) . any (| ty | contains_illegal_self_type_reference (db , trait_ , & ty , AllowSelfProjection :: Yes)) { cb (MethodViolationCode :: ReferencesSelfInput) ? ; } if contains_illegal_self_type_reference (db , trait_ , & sig . skip_binder () . output () , AllowSelfProjection :: Yes ,) { cb (MethodViolationCode :: ReferencesSelfOutput) ? ; } if ! func_data . is_async () && let Some (mvc) = contains_illegal_impl_trait_in_trait (db , & sig) { cb (mvc) ? ; } let generic_params = db . generic_params (func . into ()) ; if generic_params . len_type_or_consts () > 0 { cb (MethodViolationCode :: Generic) ? ; } if func_data . has_self_param () && ! receiver_is_dispatchable (db , trait_ , func , & sig) { cb (MethodViolationCode :: UndispatchableReceiver) ? ; } let predicates = GenericPredicates :: query_own (db , func . into ()) ; for pred in predicates . iter_identity_copied () { let pred = pred . kind () . skip_binder () ; if matches ! (pred , ClauseKind :: TypeOutlives (_)) { continue ; } if let ClauseKind :: Trait (TraitPredicate { trait_ref : pred_trait_ref , polarity : PredicatePolarity :: Positive , }) = pred && let trait_data = db . trait_signature (pred_trait_ref . def_id . 0) && trait_data . flags . contains (TraitFlags :: AUTO) && let rustc_type_ir :: TyKind :: Param (ParamTy { index : 0 , .. }) = pred_trait_ref . self_ty () . kind () { continue ; } if contains_illegal_self_type_reference (db , trait_ , & pred , AllowSelfProjection :: Yes) { cb (MethodViolationCode :: WhereClauseReferencesSelf) ? ; break ; } } ControlFlow :: Continue (()) }
    };
}

virtual_call_violations_for_method!()