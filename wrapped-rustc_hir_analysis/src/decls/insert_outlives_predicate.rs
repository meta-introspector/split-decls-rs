macro_rules! deps {
    () => {
        RequiredPredicates!();
    };
}

macro_rules! insert_outlives_predicate {
    () => {
        deps!();
        # [doc = " Given a requirement `T: 'a` or `'b: 'a`, deduce the"] # [doc = " outlives_component and add it to `required_predicates`"] pub (crate) fn insert_outlives_predicate < 'tcx > (tcx : TyCtxt < 'tcx > , arg : GenericArg < 'tcx > , outlived_region : Region < 'tcx > , span : Span , required_predicates : & mut RequiredPredicates < 'tcx > ,) { if ! is_free_region (outlived_region) { return ; } match arg . kind () { GenericArgKind :: Type (ty) => { let mut components = smallvec ! [] ; push_outlives_components (tcx , ty , & mut components) ; for component in components { match component { Component :: Region (r) => { insert_outlives_predicate (tcx , r . into () , outlived_region , span , required_predicates ,) ; } Component :: Param (param_ty) => { let ty : Ty < 'tcx > = param_ty . to_ty (tcx) ; required_predicates . entry (ty :: OutlivesPredicate (ty . into () , outlived_region)) . or_insert (span) ; } Component :: Placeholder (_) => { span_bug ! (span , "Should not deduce placeholder outlives component") ; } Component :: Alias (alias_ty) => { let ty = alias_ty . to_ty (tcx) ; required_predicates . entry (ty :: OutlivesPredicate (ty . into () , outlived_region)) . or_insert (span) ; } Component :: EscapingAlias (_) => { } Component :: UnresolvedInferenceVariable (_) => bug ! ("not using infcx") , } } } GenericArgKind :: Lifetime (r) => { if ! is_free_region (r) { return ; } required_predicates . entry (ty :: OutlivesPredicate (arg , outlived_region)) . or_insert (span) ; } GenericArgKind :: Const (_) => { } } }
    };
}

insert_outlives_predicate!()