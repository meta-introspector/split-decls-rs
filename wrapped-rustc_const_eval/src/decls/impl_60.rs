macro_rules! deps {
    () => {
        Qualif!();
        NeedsNonConstDrop!();
        ConstCx!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl Qualif for NeedsNonConstDrop { const ANALYSIS_NAME : & 'static str = "flow_needs_nonconst_drop" ; const IS_CLEARED_ON_MOVE : bool = true ; const ALLOW_PROMOTED : bool = true ; fn in_qualifs (qualifs : & ConstQualifs) -> bool { qualifs . needs_non_const_drop } # [instrument (level = "trace" , skip (cx) , ret)] fn in_any_value_of_ty < 'tcx > (cx : & ConstCx < '_ , 'tcx > , ty : Ty < 'tcx >) -> bool { if ! ty . needs_drop (cx . tcx , cx . typing_env) { return false ; } let destruct_def_id = cx . tcx . require_lang_item (LangItem :: Destruct , cx . body . span) ; let (infcx , param_env) = cx . tcx . infer_ctxt () . build_with_typing_env (cx . typing_env) ; let ocx = ObligationCtxt :: new (& infcx) ; ocx . register_obligation (Obligation :: new (cx . tcx , ObligationCause :: misc (cx . body . span , cx . def_id ()) , param_env , ty :: Binder :: dummy (ty :: TraitRef :: new (cx . tcx , destruct_def_id , [ty])) . to_host_effect_clause (cx . tcx , match cx . const_kind () { rustc_hir :: ConstContext :: ConstFn => ty :: BoundConstness :: Maybe , rustc_hir :: ConstContext :: Static (_) | rustc_hir :: ConstContext :: Const { .. } => ty :: BoundConstness :: Const , } ,) ,)) ; ! ocx . select_all_or_error () . is_empty () } fn is_structural_in_adt_value < 'tcx > (cx : & ConstCx < '_ , 'tcx > , adt : AdtDef < 'tcx >) -> bool { ! adt . has_dtor (cx . tcx) } }
    };
}

impl_60!()