mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_middle :: ty :: outlives :: { Component , push_outlives_components } ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArg , GenericArgKind , Region , Ty , TyCtxt } ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use smallvec :: smallvec ;}
mkitem!{# [doc = " Tracks the `T: 'a` or `'a: 'a` predicates that we have inferred"] # [doc = " must be added to the struct header."] pub (crate) type RequiredPredicates < 'tcx > = FxIndexMap < ty :: ArgOutlivesPredicate < 'tcx > , Span > ;}

macro_rules! insert_outlives_predicate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function insert_outlives_predicate in module {}", module_path!());
    };
}

mkfn!{
    insert_outlives_predicate_introspect!();
    # [doc = " Given a requirement `T: 'a` or `'b: 'a`, deduce the"] # [doc = " outlives_component and add it to `required_predicates`"] pub (crate) fn insert_outlives_predicate < 'tcx > (tcx : TyCtxt < 'tcx > , arg : GenericArg < 'tcx > , outlived_region : Region < 'tcx > , span : Span , required_predicates : & mut RequiredPredicates < 'tcx > ,) { if ! is_free_region (outlived_region) { return ; } match arg . kind () { GenericArgKind :: Type (ty) => { let mut components = smallvec ! [] ; push_outlives_components (tcx , ty , & mut components) ; for component in components { match component { Component :: Region (r) => { insert_outlives_predicate (tcx , r . into () , outlived_region , span , required_predicates ,) ; } Component :: Param (param_ty) => { let ty : Ty < 'tcx > = param_ty . to_ty (tcx) ; required_predicates . entry (ty :: OutlivesPredicate (ty . into () , outlived_region)) . or_insert (span) ; } Component :: Placeholder (_) => { span_bug ! (span , "Should not deduce placeholder outlives component") ; } Component :: Alias (alias_ty) => { let ty = alias_ty . to_ty (tcx) ; required_predicates . entry (ty :: OutlivesPredicate (ty . into () , outlived_region)) . or_insert (span) ; } Component :: EscapingAlias (_) => { } Component :: UnresolvedInferenceVariable (_) => bug ! ("not using infcx") , } } } GenericArgKind :: Lifetime (r) => { if ! is_free_region (r) { return ; } required_predicates . entry (ty :: OutlivesPredicate (arg , outlived_region)) . or_insert (span) ; } GenericArgKind :: Const (_) => { } } }
}

macro_rules! is_free_region_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_free_region in module {}", module_path!());
    };
}

mkfn!{
    is_free_region_introspect!();
    fn is_free_region (region : Region < '_ >) -> bool { match region . kind () { ty :: ReEarlyParam (_) => true , ty :: ReStatic => false , ty :: ReBound (..) => false , ty :: ReError (_) => false , ty :: ReErased | ty :: ReVar (..) | ty :: RePlaceholder (..) | ty :: ReLateParam (..) => { bug ! ("unexpected region in outlives inference: {:?}" , region) ; } } }
}