macro_rules! deps {
    () => {
        LateContext!();
        UntranslatableDiag!();
        DiagOutOfImpl!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        impl Diagnostics { fn is_diag_message < 'cx > (cx : & LateContext < 'cx > , ty : Ty < 'cx >) -> bool { if let Some (adt_def) = ty . ty_adt_def () && let Some (name) = cx . tcx . get_diagnostic_name (adt_def . did ()) && matches ! (name , sym :: DiagMessage | sym :: SubdiagMessage) { true } else { false } } fn untranslatable_diagnostic < 'cx > (cx : & LateContext < 'cx > , def_id : DefId , arg_tys_and_spans : & [(Ty < 'cx > , Span)] ,) { let fn_sig = cx . tcx . fn_sig (def_id) . instantiate_identity () . skip_binder () ; let predicates = cx . tcx . predicates_of (def_id) . instantiate_identity (cx . tcx) . predicates ; for (i , & param_ty) in fn_sig . inputs () . iter () . enumerate () { if let ty :: Param (sig_param) = param_ty . kind () { for pred in predicates . iter () { if let Some (trait_pred) = pred . as_trait_clause () && let trait_ref = trait_pred . skip_binder () . trait_ref && trait_ref . self_ty () == param_ty && cx . tcx . is_diagnostic_item (sym :: Into , trait_ref . def_id) && let ty1 = trait_ref . args . type_at (1) && Self :: is_diag_message (cx , ty1) { let (arg_ty , arg_span) = arg_tys_and_spans [i] ; let is_translatable = Self :: is_diag_message (cx , arg_ty) || matches ! (arg_ty . kind () , ty :: Param (arg_param) if arg_param . name == sig_param . name) ; if ! is_translatable { cx . emit_span_lint (UNTRANSLATABLE_DIAGNOSTIC , arg_span , UntranslatableDiag ,) ; } } } } } } fn diagnostic_outside_of_impl < 'cx > (cx : & LateContext < 'cx > , span : Span , current_id : HirId , def_id : DefId , fn_gen_args : GenericArgsRef < 'cx > ,) { let Some (inst) = ty :: Instance :: try_resolve (cx . tcx , cx . typing_env () , def_id , fn_gen_args) . ok () . flatten () else { return ; } ; let has_attr = cx . tcx . has_attr (inst . def_id () , sym :: rustc_lint_diagnostics) ; if ! has_attr { return ; } ; for (hir_id , _parent) in cx . tcx . hir_parent_iter (current_id) { if let Some (owner_did) = hir_id . as_owner () && cx . tcx . has_attr (owner_did , sym :: rustc_lint_diagnostics) { return ; } } let mut is_inside_appropriate_impl = false ; for (_hir_id , parent) in cx . tcx . hir_parent_iter (current_id) { debug ! (? parent) ; if let hir :: Node :: Item (hir :: Item { kind : hir :: ItemKind :: Impl (impl_) , .. }) = parent && let Some (of_trait) = impl_ . of_trait && let Some (def_id) = of_trait . trait_ref . trait_def_id () && let Some (name) = cx . tcx . get_diagnostic_name (def_id) && matches ! (name , sym :: Diagnostic | sym :: Subdiagnostic | sym :: LintDiagnostic) { is_inside_appropriate_impl = true ; break ; } } debug ! (? is_inside_appropriate_impl) ; if ! is_inside_appropriate_impl { cx . emit_span_lint (DIAGNOSTIC_OUTSIDE_OF_IMPL , span , DiagOutOfImpl) ; } } }
    };
}

impl_292!()