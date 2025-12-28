macro_rules! deps {
    () => {
        ExplicitPredicatesMap!();
        RequiredPredicates!();
    };
}

macro_rules! check_explicit_predicates {
    () => {
        deps!();
        # [doc = " Check the explicit predicates declared on the type."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```ignore (illustrative)"] # [doc = " struct Outer<'a, T> {"] # [doc = "     field: Inner<T>,"] # [doc = " }"] # [doc = ""] # [doc = " struct Inner<U> where U: 'static, U: Outer {"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] # [doc = " Here, we should fetch the explicit predicates, which"] # [doc = " will give us `U: 'static` and `U: Outer`. The latter we"] # [doc = " can ignore, but we will want to process `U: 'static`,"] # [doc = " applying the instantiation as above."] fn check_explicit_predicates < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId , args : & [GenericArg < 'tcx >] , required_predicates : & mut RequiredPredicates < 'tcx > , explicit_map : & mut ExplicitPredicatesMap < 'tcx > , ignored_self_ty : Option < Ty < 'tcx > > ,) { debug ! ("check_explicit_predicates(def_id={:?}, \
         args={:?}, \
         explicit_map={:?}, \
         required_predicates={:?}, \
         ignored_self_ty={:?})" , def_id , args , explicit_map , required_predicates , ignored_self_ty ,) ; let explicit_predicates = explicit_map . explicit_predicates_of (tcx , def_id) ; for (outlives_predicate , & span) in explicit_predicates . as_ref () . skip_binder () { debug ! ("outlives_predicate = {outlives_predicate:?}") ; if let Some (self_ty) = ignored_self_ty && let GenericArgKind :: Type (ty) = outlives_predicate . 0 . kind () && ty . walk () . any (| arg | arg == self_ty . into ()) { debug ! ("skipping self ty = {ty:?}") ; continue ; } let predicate = explicit_predicates . rebind (* outlives_predicate) . instantiate (tcx , args) ; debug ! ("predicate = {predicate:?}") ; insert_outlives_predicate (tcx , predicate . 0 , predicate . 1 , span , required_predicates) ; } }
    };
}

check_explicit_predicates!()