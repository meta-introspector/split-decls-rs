macro_rules! early_bound_lifetimes_from_generics {
    () => {
        # [doc = " Returns the early-bound lifetimes declared in this generics"] # [doc = " listing. For anything other than fns/methods, this is just all"] # [doc = " the lifetimes that are declared. For fns or methods, we have to"] # [doc = " screen out those that do not appear in any where-clauses etc using"] # [doc = " `resolve_lifetime::early_bound_lifetimes`."] fn early_bound_lifetimes_from_generics < 'a , 'tcx > (tcx : TyCtxt < 'tcx > , generics : & 'a hir :: Generics < 'a > ,) -> impl Iterator < Item = & 'a hir :: GenericParam < 'a > > { generics . params . iter () . filter (move | param | match param . kind { GenericParamKind :: Lifetime { .. } => ! tcx . is_late_bound (param . hir_id) , _ => false , }) }
    };
}

early_bound_lifetimes_from_generics!()