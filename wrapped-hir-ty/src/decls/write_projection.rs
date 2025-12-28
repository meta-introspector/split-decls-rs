macro_rules! deps {
    () => {
        HirDisplayError!();
        AliasTy!();
        SizedByDefault!();
        HirFormatter!();
        GenericPredicates!();
    };
}

macro_rules! write_projection {
    () => {
        deps!();
        fn write_projection < 'db > (f : & mut HirFormatter < '_ , 'db > , alias : & AliasTy < 'db > ,) -> Result < () , HirDisplayError > { if f . should_truncate () { return write ! (f , "{TYPE_HINT_TRUNCATION}") ; } let trait_ref = alias . trait_ref (f . interner) ; let self_ty = trait_ref . self_ty () ; if ! f . display_kind . is_source_code () && let TyKind :: Param (param) = self_ty . kind () && ! f . bounds_formatting_ctx . contains (alias) { let bounds = GenericPredicates :: query_all (f . db , param . id . parent ()) . iter_identity_copied () . filter (| wc | { let ty = match wc . kind () . skip_binder () { ClauseKind :: Trait (tr) => tr . self_ty () , ClauseKind :: TypeOutlives (t) => t . 0 , _ => return false , } ; let TyKind :: Alias (AliasTyKind :: Projection , a) = ty . kind () else { return false ; } ; a == * alias }) . collect :: < Vec < _ > > () ; if ! bounds . is_empty () { return f . format_bounds_with (* alias , | f | { write_bounds_like_dyn_trait_with_prefix (f , "impl" , Either :: Left (Ty :: new_alias (f . interner , AliasTyKind :: Projection , * alias)) , & bounds , SizedByDefault :: NotSized ,) }) ; } } write ! (f , "<") ? ; self_ty . hir_fmt (f) ? ; write ! (f , " as ") ? ; trait_ref . hir_fmt (f) ? ; write ! (f , ">::{}" , f . db . type_alias_signature (alias . def_id . expect_type_alias ()) . name . display (f . db , f . edition ())) ? ; let proj_params = & alias . args . as_slice () [trait_ref . args . len () ..] ; hir_fmt_generics (f , proj_params , None , None) }
    };
}

write_projection!()