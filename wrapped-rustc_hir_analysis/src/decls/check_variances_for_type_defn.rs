macro_rules! deps {
    () => {
        Parameter!();
        ItemCtxt!();
        HasErrorDeep!();
    };
}

macro_rules! check_variances_for_type_defn {
    () => {
        deps!();
        pub (super) fn check_variances_for_type_defn < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : LocalDefId) { match tcx . def_kind (def_id) { DefKind :: Enum | DefKind :: Struct | DefKind :: Union => { } DefKind :: TyAlias => { assert ! (tcx . type_alias_is_lazy (def_id) , "should not be computing variance of non-free type alias") ; } kind => span_bug ! (tcx . def_span (def_id) , "cannot compute the variances of {kind:?}") , } let ty_predicates = tcx . predicates_of (def_id) ; assert_eq ! (ty_predicates . parent , None) ; let variances = tcx . variances_of (def_id) ; let mut constrained_parameters : FxHashSet < _ > = variances . iter () . enumerate () . filter (| & (_ , & variance) | variance != ty :: Bivariant) . map (| (index , _) | Parameter (index as u32)) . collect () ; identify_constrained_generic_params (tcx , ty_predicates , None , & mut constrained_parameters) ; let explicitly_bounded_params = LazyCell :: new (| | { let icx = crate :: collect :: ItemCtxt :: new (tcx , def_id) ; tcx . hir_node_by_def_id (def_id) . generics () . unwrap () . predicates . iter () . filter_map (| predicate | match predicate . kind { hir :: WherePredicateKind :: BoundPredicate (predicate) => { match icx . lower_ty (predicate . bounded_ty) . kind () { ty :: Param (data) => Some (Parameter (data . index)) , _ => None , } } _ => None , }) . collect :: < FxHashSet < _ > > () }) ; for (index , _) in variances . iter () . enumerate () { let parameter = Parameter (index as u32) ; if constrained_parameters . contains (& parameter) { continue ; } let node = tcx . hir_node_by_def_id (def_id) ; let item = node . expect_item () ; let hir_generics = node . generics () . unwrap () ; let hir_param = & hir_generics . params [index] ; let ty_param = & tcx . generics_of (item . owner_id) . own_params [index] ; if ty_param . def_id != hir_param . def_id . into () { tcx . dcx () . span_delayed_bug (hir_param . span , "hir generics and ty generics in different order" ,) ; continue ; } if let ControlFlow :: Break (ErrorGuaranteed { .. }) = tcx . type_of (def_id) . instantiate_identity () . visit_with (& mut HasErrorDeep { tcx , seen : Default :: default () }) { continue ; } match hir_param . name { hir :: ParamName :: Error (_) => { } _ => { let has_explicit_bounds = explicitly_bounded_params . contains (& parameter) ; report_bivariance (tcx , hir_param , has_explicit_bounds , item) ; } } } }
    };
}

check_variances_for_type_defn!()