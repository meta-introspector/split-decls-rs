macro_rules! deps {
    () => {
        HirDatabase!();
        Binder!();
    };
}

macro_rules! implicitly_sized_clauses {
    () => {
        deps!();
        # [doc = " Generate implicit `: Sized` predicates for all generics that has no `?Sized` bound."] # [doc = " Exception is Self of a trait def."] fn implicitly_sized_clauses < 'a , 'subst , 'db > (db : & 'db dyn HirDatabase , def : GenericDefId , explicitly_unsized_tys : & 'a FxHashSet < Ty < 'db > > , args : & 'subst GenericArgs < 'db > , resolver : & Resolver < 'db > ,) -> Option < impl Iterator < Item = Clause < 'db > > + Captures < 'a > + Captures < 'subst > > { let interner = DbInterner :: new_with (db , Some (resolver . krate ()) , None) ; let sized_trait = LangItem :: Sized . resolve_trait (db , resolver . krate ()) ? ; let trait_self_idx = trait_self_param_idx (db , def) ; Some (args . iter () . enumerate () . filter_map (move | (idx , generic_arg) | { if Some (idx) == trait_self_idx { None } else { Some (generic_arg) } } ,) . filter_map (| generic_arg | generic_arg . as_type ()) . filter (move | self_ty | ! explicitly_unsized_tys . contains (self_ty)) . map (move | self_ty | { let trait_ref = TraitRef :: new_from_args (interner , sized_trait . into () , GenericArgs :: new_from_iter (interner , [self_ty . into ()]) ,) ; Clause (Predicate :: new (interner , Binder :: dummy (rustc_type_ir :: PredicateKind :: Clause (rustc_type_ir :: ClauseKind :: Trait (TraitPredicate { trait_ref , polarity : rustc_type_ir :: PredicatePolarity :: Positive , }) ,)) ,)) }) ,) }
    };
}

implicitly_sized_clauses!();