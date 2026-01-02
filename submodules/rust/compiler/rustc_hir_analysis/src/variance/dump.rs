mkuse!{use std :: fmt :: Write ;}
mkuse!{use rustc_hir :: def_id :: { CRATE_DEF_ID , LocalDefId } ;}
mkuse!{use rustc_middle :: ty :: { GenericArgs , TyCtxt } ;}
mkuse!{use rustc_span :: sym ;}

macro_rules! format_variances_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function format_variances in module {}", module_path!());
    };
}

mkfn!{
    format_variances_introspect!();
    fn format_variances (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> String { let variances = tcx . variances_of (def_id) ; let generics = GenericArgs :: identity_for_item (tcx , def_id) ; let mut ret = String :: with_capacity (2 + 7 * variances . len ()) ; ret . push ('[') ; for (arg , variance) in generics . iter () . zip (variances . iter ()) { write ! (ret , "{arg}: {variance:?}, ") . unwrap () ; } if ! variances . is_empty () { ret . pop () ; ret . pop () ; } ret . push (']') ; ret }
}

macro_rules! variances_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function variances in module {}", module_path!());
    };
}

mkfn!{
    variances_introspect!();
    pub (crate) fn variances (tcx : TyCtxt < '_ >) { let crate_items = tcx . hir_crate_items (()) ; if tcx . has_attr (CRATE_DEF_ID , sym :: rustc_variance_of_opaques) { for id in crate_items . opaques () { tcx . dcx () . emit_err (crate :: errors :: VariancesOf { span : tcx . def_span (id) , variances : format_variances (tcx , id) , }) ; } } for id in crate_items . free_items () { if ! tcx . has_attr (id . owner_id , sym :: rustc_variance) { continue ; } tcx . dcx () . emit_err (crate :: errors :: VariancesOf { span : tcx . def_span (id . owner_id) , variances : format_variances (tcx , id . owner_id . def_id) , }) ; } }
}