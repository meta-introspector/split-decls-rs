macro_rules! deps {
    () => {
        RequiredPredicates!();
        ExplicitPredicatesMap!();
    };
}

macro_rules! insert_required_predicates_to_be_wf {
    () => {
        deps!();
        fn insert_required_predicates_to_be_wf < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , span : Span , global_inferred_outlives : & FxIndexMap < DefId , ty :: EarlyBinder < 'tcx , RequiredPredicates < 'tcx > > > , required_predicates : & mut RequiredPredicates < 'tcx > , explicit_map : & mut ExplicitPredicatesMap < 'tcx > ,) { for arg in ty . walk () { let leaf_ty = match arg . kind () { GenericArgKind :: Type (ty) => ty , GenericArgKind :: Lifetime (_) | GenericArgKind :: Const (_) => continue , } ; match * leaf_ty . kind () { ty :: Ref (region , rty , _) => { debug ! ("Ref") ; insert_outlives_predicate (tcx , rty . into () , region , span , required_predicates) ; } ty :: Adt (def , args) => { debug ! ("Adt") ; check_inferred_predicates (tcx , def . did () , args , global_inferred_outlives , required_predicates ,) ; check_explicit_predicates (tcx , def . did () , args , required_predicates , explicit_map , None ,) ; } ty :: Alias (ty :: Free , alias) => { debug ! ("Free") ; check_inferred_predicates (tcx , alias . def_id , alias . args , global_inferred_outlives , required_predicates ,) ; check_explicit_predicates (tcx , alias . def_id , alias . args , required_predicates , explicit_map , None ,) ; } ty :: Dynamic (obj , ..) => { debug ! ("Dynamic") ; if let Some (ex_trait_ref) = obj . principal () { let args = ex_trait_ref . with_self_ty (tcx , tcx . types . usize) . skip_binder () . args ; check_explicit_predicates (tcx , ex_trait_ref . skip_binder () . def_id , args , required_predicates , explicit_map , Some (tcx . types . self_param) ,) ; } } ty :: Alias (ty :: Projection , alias) => { debug ! ("Projection") ; check_explicit_predicates (tcx , tcx . parent (alias . def_id) , alias . args , required_predicates , explicit_map , None ,) ; } ty :: Alias (ty :: Inherent , _) => { } _ => { } } } }
    };
}

insert_required_predicates_to_be_wf!();