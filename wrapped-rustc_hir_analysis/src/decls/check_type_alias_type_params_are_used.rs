macro_rules! deps {
    () => {
        UnusedGenericParameter!();
        UnusedGenericParameterHelp!();
    };
}

macro_rules! check_type_alias_type_params_are_used {
    () => {
        deps!();
        fn check_type_alias_type_params_are_used < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId) { if tcx . type_alias_is_lazy (def_id) { return ; } let generics = tcx . generics_of (def_id) ; if generics . own_counts () . types == 0 { return ; } let ty = tcx . type_of (def_id) . instantiate_identity () ; if ty . references_error () { return ; } let bounded_params = LazyCell :: new (| | { tcx . explicit_predicates_of (def_id) . predicates . iter () . filter_map (| (predicate , span) | { let bounded_ty = match predicate . kind () . skip_binder () { ty :: ClauseKind :: Trait (pred) => pred . trait_ref . self_ty () , ty :: ClauseKind :: TypeOutlives (pred) => pred . 0 , _ => return None , } ; if let ty :: Param (param) = bounded_ty . kind () { Some ((param . index , span)) } else { None } }) . collect :: < FxIndexMap < _ , _ > > () }) ; let mut params_used = DenseBitSet :: new_empty (generics . own_params . len ()) ; for leaf in ty . walk () { if let GenericArgKind :: Type (leaf_ty) = leaf . kind () && let ty :: Param (param) = leaf_ty . kind () { debug ! ("found use of ty param {:?}" , param) ; params_used . insert (param . index) ; } } for param in & generics . own_params { if ! params_used . contains (param . index) && let ty :: GenericParamDefKind :: Type { .. } = param . kind { let span = tcx . def_span (param . def_id) ; let param_name = Ident :: new (param . name , span) ; let has_explicit_bounds = bounded_params . is_empty () || (* bounded_params) . get (& param . index) . is_some_and (| & & pred_sp | pred_sp != span) ; let const_param_help = ! has_explicit_bounds ; let mut diag = tcx . dcx () . create_err (errors :: UnusedGenericParameter { span , param_name , param_def_kind : tcx . def_descr (param . def_id) , help : errors :: UnusedGenericParameterHelp :: TyAlias { param_name } , usage_spans : vec ! [] , const_param_help , }) ; diag . code (E0091) ; diag . emit () ; } } }
    };
}

check_type_alias_type_params_are_used!();