mkmod!{check, { 
                getname!(check);
                getsrc!(check);
                getpath!(check);
                get_deps!(check);
                get_crates!(check);
                mkinclude!(check);
                 
            }}
mkmod!{autoderef, { 
                getname!(autoderef);
                getsrc!(autoderef);
                getpath!(autoderef);
                get_deps!(autoderef);
                get_crates!(autoderef);
                mkinclude!(autoderef);
                 
            }}
mkmod!{check_unused, { 
                getname!(check_unused);
                getsrc!(check_unused);
                getpath!(check_unused);
                get_deps!(check_unused);
                get_crates!(check_unused);
                mkinclude!(check_unused);
                 
            }}
mkmod!{coherence, { 
                getname!(coherence);
                getsrc!(coherence);
                getpath!(coherence);
                get_deps!(coherence);
                get_crates!(coherence);
                mkinclude!(coherence);
                 
            }}
mkmod!{collect, { 
                getname!(collect);
                getsrc!(collect);
                getpath!(collect);
                get_deps!(collect);
                get_crates!(collect);
                mkinclude!(collect);
                 
            }}
mkmod!{constrained_generic_params, { 
                getname!(constrained_generic_params);
                getsrc!(constrained_generic_params);
                getpath!(constrained_generic_params);
                get_deps!(constrained_generic_params);
                get_crates!(constrained_generic_params);
                mkinclude!(constrained_generic_params);
                 
            }}
mkmod!{delegation, { 
                getname!(delegation);
                getsrc!(delegation);
                getpath!(delegation);
                get_deps!(delegation);
                get_crates!(delegation);
                mkinclude!(delegation);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{hir_ty_lowering, { 
                getname!(hir_ty_lowering);
                getsrc!(hir_ty_lowering);
                getpath!(hir_ty_lowering);
                get_deps!(hir_ty_lowering);
                get_crates!(hir_ty_lowering);
                mkinclude!(hir_ty_lowering);
                 
            }}
mkmod!{hir_wf_check, { 
                getname!(hir_wf_check);
                getsrc!(hir_wf_check);
                getpath!(hir_wf_check);
                get_deps!(hir_wf_check);
                get_crates!(hir_wf_check);
                mkinclude!(hir_wf_check);
                 
            }}
mkmod!{impl_wf_check, { 
                getname!(impl_wf_check);
                getsrc!(impl_wf_check);
                getpath!(impl_wf_check);
                get_deps!(impl_wf_check);
                get_crates!(impl_wf_check);
                mkinclude!(impl_wf_check);
                 
            }}
mkmod!{outlives, { 
                getname!(outlives);
                getsrc!(outlives);
                getpath!(outlives);
                get_deps!(outlives);
                get_crates!(outlives);
                mkinclude!(outlives);
                 
            }}
mkmod!{variance, { 
                getname!(variance);
                getsrc!(variance);
                getpath!(variance);
                get_deps!(variance);
                get_crates!(variance);
                mkinclude!(variance);
                 
            }}
mkuse!{pub use errors :: NoVariantNamed ;}
mkuse!{use rustc_abi :: { CVariadicStatus , ExternAbi } ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: lints :: DelayedLint ;}
mkuse!{use rustc_hir :: { self as hir } ;}
mkuse!{use rustc_middle :: middle ;}
mkuse!{use rustc_middle :: mir :: interpret :: GlobalId ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { self , Const , Ty , TyCtxt } ;}
mkuse!{use rustc_session :: parse :: feature_err ;}
mkuse!{use rustc_span :: { ErrorGuaranteed , Span } ;}
mkuse!{use rustc_trait_selection :: traits ;}
mkuse!{pub use crate :: collect :: suggest_impl_trait ;}
mkuse!{use crate :: hir_ty_lowering :: { FeedConstTy , HirTyLowerer } ;}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}

macro_rules! check_c_variadic_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_c_variadic_abi in module {}", module_path!());
    };
}

mkfn!{
    check_c_variadic_abi_introspect!();
    fn check_c_variadic_abi (tcx : TyCtxt < '_ > , decl : & hir :: FnDecl < '_ > , abi : ExternAbi , span : Span) { if ! decl . c_variadic { return ; } match abi . supports_c_variadic () { CVariadicStatus :: Stable => { } CVariadicStatus :: NotSupported => { tcx . dcx () . create_err (errors :: VariadicFunctionCompatibleConvention { span , convention : & format ! ("{abi}") , }) . emit () ; } CVariadicStatus :: Unstable { feature } => { if ! tcx . features () . enabled (feature) { feature_err (& tcx . sess , feature , span , format ! ("C-variadic functions with the {abi} calling convention are unstable") ,) . emit () ; } } } }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    # [doc = " Adds query implementations to the [Providers] vtable, see [`rustc_middle::query`]"] pub fn provide (providers : & mut Providers) { collect :: provide (providers) ; coherence :: provide (providers) ; check :: provide (providers) ; * providers = Providers { check_unused_traits : check_unused :: check_unused_traits , diagnostic_hir_wf_check : hir_wf_check :: diagnostic_hir_wf_check , inferred_outlives_crate : outlives :: inferred_outlives_crate , inferred_outlives_of : outlives :: inferred_outlives_of , inherit_sig_for_delegation_item : delegation :: inherit_sig_for_delegation_item , enforce_impl_non_lifetime_params_are_constrained : impl_wf_check :: enforce_impl_non_lifetime_params_are_constrained , crate_variances : variance :: crate_variances , variances_of : variance :: variances_of , .. * providers } ; }
}

macro_rules! emit_delayed_lint_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function emit_delayed_lint in module {}", module_path!());
    };
}

mkfn!{
    emit_delayed_lint_introspect!();
    fn emit_delayed_lint (lint : & DelayedLint , tcx : TyCtxt < '_ >) { match lint { DelayedLint :: AttributeParsing (attribute_lint) => { rustc_attr_parsing :: emit_attribute_lint (attribute_lint , tcx) } } }
}

macro_rules! check_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_crate in module {}", module_path!());
    };
}

mkfn!{
    check_crate_introspect!();
    pub fn check_crate (tcx : TyCtxt < '_ >) { let _prof_timer = tcx . sess . timer ("type_check_crate") ; tcx . sess . time ("coherence_checking" , | | { type R = Result < () , ErrorGuaranteed > ; let _ : R = tcx . ensure_ok () . check_type_wf (()) ; for & trait_def_id in tcx . all_local_trait_impls (()) . keys () { let _ : R = tcx . ensure_ok () . coherent_trait (trait_def_id) ; } let _ : R = tcx . ensure_ok () . crate_inherent_impls_validity_check (()) ; let _ : R = tcx . ensure_ok () . crate_inherent_impls_overlap_check (()) ; }) ; tcx . sess . time ("emit_ast_lowering_delayed_lints" , | | { # [cfg (debug_assertions)] { for owner_id in tcx . hir_crate_items (()) . owners () { if let Some (delayed_lints) = tcx . opt_ast_lowering_delayed_lints (owner_id) { if ! delayed_lints . lints . is_empty () { assert ! (tcx . hir_crate_items (()) . delayed_lint_items () . any (| i | i == owner_id)) ; } } } } for owner_id in tcx . hir_crate_items (()) . delayed_lint_items () { if let Some (delayed_lints) = tcx . opt_ast_lowering_delayed_lints (owner_id) { for lint in & delayed_lints . lints { emit_delayed_lint (lint , tcx) ; } } } }) ; tcx . par_hir_body_owners (| item_def_id | { let def_kind = tcx . def_kind (item_def_id) ; match def_kind { DefKind :: Static { .. } => { tcx . ensure_ok () . eval_static_initializer (item_def_id) ; check :: maybe_check_static_with_link_section (tcx , item_def_id) ; } DefKind :: Const if ! tcx . generics_of (item_def_id) . own_requires_monomorphization () => { let instance = ty :: Instance :: new_raw (item_def_id . into () , ty :: GenericArgs :: empty ()) ; let cid = GlobalId { instance , promoted : None } ; let typing_env = ty :: TypingEnv :: fully_monomorphized () ; tcx . ensure_ok () . eval_to_const_value_raw (typing_env . as_query_input (cid)) ; } _ => () , } if ! (matches ! (def_kind , DefKind :: AnonConst) || def_kind . is_typeck_child ()) { tcx . ensure_ok () . typeck (item_def_id) ; } if tcx . needs_coroutine_by_move_body_def_id (item_def_id . to_def_id ()) { tcx . ensure_done () . coroutine_by_move_body_def_id (item_def_id) ; } }) ; if tcx . features () . rustc_attrs () { tcx . sess . time ("dumping_rustc_attr_data" , | | { outlives :: dump :: inferred_outlives (tcx) ; variance :: dump :: variances (tcx) ; collect :: dump :: opaque_hidden_types (tcx) ; collect :: dump :: predicates_and_item_bounds (tcx) ; collect :: dump :: def_parents (tcx) ; collect :: dump :: vtables (tcx) ; }) ; } tcx . ensure_ok () . check_unused_traits (()) ; }
}

macro_rules! lower_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lower_ty in module {}", module_path!());
    };
}

mkfn!{
    lower_ty_introspect!();
    # [doc = " Lower a [`hir::Ty`] to a [`Ty`]."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " This function is **quasi-deprecated**. It can cause ICEs if called inside of a body"] # [doc = " (of a function or constant) and especially if it contains inferred types (`_`)."] # [doc = ""] # [doc = " It's used in rustdoc and Clippy."] # [doc = ""] # [doc = " </div>"] pub fn lower_ty < 'tcx > (tcx : TyCtxt < 'tcx > , hir_ty : & hir :: Ty < 'tcx >) -> Ty < 'tcx > { let env_def_id = tcx . hir_get_parent_item (hir_ty . hir_id) ; collect :: ItemCtxt :: new (tcx , env_def_id . def_id) . lowerer () . lower_ty_maybe_return_type_notation (hir_ty) }
}

macro_rules! lower_const_arg_for_rustdoc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lower_const_arg_for_rustdoc in module {}", module_path!());
    };
}

mkfn!{
    lower_const_arg_for_rustdoc_introspect!();
    # [doc = " This is for rustdoc."] pub fn lower_const_arg_for_rustdoc < 'tcx > (tcx : TyCtxt < 'tcx > , hir_ct : & hir :: ConstArg < 'tcx > , feed : FeedConstTy < '_ , 'tcx > ,) -> Const < 'tcx > { let env_def_id = tcx . hir_get_parent_item (hir_ct . hir_id) ; collect :: ItemCtxt :: new (tcx , env_def_id . def_id) . lowerer () . lower_const_arg (hir_ct , feed) }
}