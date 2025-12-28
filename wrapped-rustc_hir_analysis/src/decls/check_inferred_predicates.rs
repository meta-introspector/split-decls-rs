macro_rules! deps {
    () => {
        RequiredPredicates!();
    };
}

macro_rules! check_inferred_predicates {
    () => {
        deps!();
        # [doc = " Check the inferred predicates declared on the type."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " struct Outer<'a, T> {"] # [doc = "     outer: Inner<'a, T>,"] # [doc = " }"] # [doc = ""] # [doc = " struct Inner<'b, U> {"] # [doc = "     inner: &'b U,"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Here, when processing the type of field `outer`, we would request the"] # [doc = " set of implicit predicates computed for `Inner` thus far. This will"] # [doc = " initially come back empty, but in next round we will get `U: 'b`."] # [doc = " We then apply the instantiation `['b => 'a, U => T]` and thus get the"] # [doc = " requirement that `T: 'a` holds for `Outer`."] fn check_inferred_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , args : ty :: GenericArgsRef < 'tcx > , global_inferred_outlives : & FxIndexMap < DefId , ty :: EarlyBinder < 'tcx , RequiredPredicates < 'tcx > > > , required_predicates : & mut RequiredPredicates < 'tcx > ,) { let Some (predicates) = global_inferred_outlives . get (& def_id) else { return ; } ; for (& predicate , & span) in predicates . as_ref () . skip_binder () { let ty :: OutlivesPredicate (arg , region) = predicates . rebind (predicate) . instantiate (tcx , args) ; insert_outlives_predicate (tcx , arg , region , span , required_predicates) ; } }
    };
}

check_inferred_predicates!()