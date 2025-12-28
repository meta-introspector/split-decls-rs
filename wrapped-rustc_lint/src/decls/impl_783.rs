macro_rules! deps {
    () => {
        LateContext!();
        DropTraitConstraintsDiag!();
        DropGlue!();
    };
}

macro_rules! impl_783 {
    () => {
        deps!();
        impl < 'tcx > LateLintPass < 'tcx > for DropTraitConstraints { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx hir :: Item < 'tcx >) { use rustc_middle :: ty :: ClauseKind ; let predicates = cx . tcx . explicit_predicates_of (item . owner_id) ; for & (predicate , span) in predicates . predicates { let ClauseKind :: Trait (trait_predicate) = predicate . kind () . skip_binder () else { continue ; } ; let def_id = trait_predicate . trait_ref . def_id ; if cx . tcx . is_lang_item (def_id , LangItem :: Drop) { if trait_predicate . trait_ref . self_ty () . is_impl_trait () { continue ; } let Some (def_id) = cx . tcx . get_diagnostic_item (sym :: needs_drop) else { return } ; cx . emit_span_lint (DROP_BOUNDS , span , DropTraitConstraintsDiag { predicate , tcx : cx . tcx , def_id } ,) ; } } } fn check_ty (& mut self , cx : & LateContext < '_ > , ty : & 'tcx hir :: Ty < 'tcx , AmbigArg >) { let hir :: TyKind :: TraitObject (bounds , _lifetime_and_syntax_pointer) = & ty . kind else { return ; } ; for bound in & bounds [..] { let def_id = bound . trait_ref . trait_def_id () ; if def_id . is_some_and (| def_id | cx . tcx . is_lang_item (def_id , LangItem :: Drop)) { let Some (def_id) = cx . tcx . get_diagnostic_item (sym :: needs_drop) else { return } ; cx . emit_span_lint (DYN_DROP , bound . span , DropGlue { tcx : cx . tcx , def_id }) ; } } } }
    };
}

impl_783!()