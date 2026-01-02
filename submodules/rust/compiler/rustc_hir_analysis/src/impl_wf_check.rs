mkuse!{use std :: assert_matches :: debug_assert_matches ;}
mkuse!{use min_specialization :: check_min_specialization ;}
mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_errors :: codes :: * ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: LocalDefId ;}
mkuse!{use rustc_middle :: ty :: { self , TyCtxt , TypeVisitableExt } ;}
mkuse!{use rustc_span :: ErrorGuaranteed ;}
mkuse!{use crate :: constrained_generic_params as cgp ;}
mkuse!{use crate :: errors :: UnconstrainedGenericParameter ;}
mkmod!{min_specialization, { 
                getname!(min_specialization);
                getsrc!(min_specialization);
                getpath!(min_specialization);
                get_deps!(min_specialization);
                get_crates!(min_specialization);
                mkinclude!(min_specialization);
                 
            }}

macro_rules! check_impl_wf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_impl_wf in module {}", module_path!());
    };
}

mkfn!{
    check_impl_wf_introspect!();
    # [doc = " Checks that all the type/lifetime parameters on an impl also"] # [doc = " appear in the trait ref or self type (or are constrained by a"] # [doc = " where-clause). These rules are needed to ensure that, given a"] # [doc = " trait ref like `<T as Trait<U>>`, we can derive the values of all"] # [doc = " parameters on the impl (which is needed to make specialization"] # [doc = " possible)."] # [doc = ""] # [doc = " However, in the case of lifetimes, we only enforce these rules if"] # [doc = " the lifetime parameter is used in an associated type. This is a"] # [doc = " concession to backwards compatibility; see comment at the end of"] # [doc = " the fn for details."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```rust,ignore (pseudo-Rust)"] # [doc = " impl<T> Trait<Foo> for Bar { ... }"] # [doc = " //   ^ T does not appear in `Foo` or `Bar`, error!"] # [doc = ""] # [doc = " impl<T> Trait<Foo<T>> for Bar { ... }"] # [doc = " //   ^ T appears in `Foo<T>`, ok."] # [doc = ""] # [doc = " impl<T> Trait<Foo> for Bar where Bar: Iterator<Item = T> { ... }"] # [doc = " //   ^ T is bound to `<Bar as Iterator>::Item`, ok."] # [doc = ""] # [doc = " impl<'a> Trait<Foo> for Bar { }"] # [doc = " //   ^ 'a is unused, but for back-compat we allow it"] # [doc = ""] # [doc = " impl<'a> Trait<Foo> for Bar { type X = &'a i32; }"] # [doc = " //   ^ 'a is unused and appears in assoc type, error"] # [doc = " ```"] pub (crate) fn check_impl_wf (tcx : TyCtxt < '_ > , impl_def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { debug_assert_matches ! (tcx . def_kind (impl_def_id) , DefKind :: Impl { .. }) ; let mut res = tcx . ensure_ok () . enforce_impl_non_lifetime_params_are_constrained (impl_def_id) ; res = res . and (enforce_impl_lifetime_params_are_constrained (tcx , impl_def_id)) ; if tcx . features () . min_specialization () { res = res . and (check_min_specialization (tcx , impl_def_id)) ; } res }
}

macro_rules! enforce_impl_lifetime_params_are_constrained_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enforce_impl_lifetime_params_are_constrained in module {}", module_path!());
    };
}

mkfn!{
    enforce_impl_lifetime_params_are_constrained_introspect!();
    pub (crate) fn enforce_impl_lifetime_params_are_constrained (tcx : TyCtxt < '_ > , impl_def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { let impl_self_ty = tcx . type_of (impl_def_id) . instantiate_identity () ; if impl_self_ty . references_error () { tcx . dcx () . span_delayed_bug (tcx . def_span (impl_def_id) , format ! ("potentially unconstrained type parameters weren't evaluated: {impl_self_ty:?}" ,) ,) ; return Ok (()) ; } let impl_generics = tcx . generics_of (impl_def_id) ; let impl_predicates = tcx . predicates_of (impl_def_id) ; let impl_trait_ref = tcx . impl_trait_ref (impl_def_id) . map (ty :: EarlyBinder :: instantiate_identity) ; impl_trait_ref . error_reported () ? ; let mut input_parameters = cgp :: parameters_for_impl (tcx , impl_self_ty , impl_trait_ref) ; cgp :: identify_constrained_generic_params (tcx , impl_predicates , impl_trait_ref , & mut input_parameters ,) ; let lifetimes_in_associated_types : FxHashSet < _ > = tcx . associated_item_def_ids (impl_def_id) . iter () . flat_map (| def_id | { let item = tcx . associated_item (def_id) ; match item . kind { ty :: AssocKind :: Type { .. } => { if item . defaultness (tcx) . has_value () { cgp :: parameters_for (tcx , tcx . type_of (def_id) . instantiate_identity () , true) } else { vec ! [] } } ty :: AssocKind :: Fn { .. } | ty :: AssocKind :: Const { .. } => vec ! [] , } }) . collect () ; let mut res = Ok (()) ; for param in & impl_generics . own_params { match param . kind { ty :: GenericParamDefKind :: Lifetime => { let param_lt = cgp :: Parameter :: from (param . to_early_bound_region_data ()) ; if lifetimes_in_associated_types . contains (& param_lt) && ! input_parameters . contains (& param_lt) { let mut diag = tcx . dcx () . create_err (UnconstrainedGenericParameter { span : tcx . def_span (param . def_id) , param_name : tcx . item_ident (param . def_id) , param_def_kind : tcx . def_descr (param . def_id) , const_param_note : false , const_param_note2 : false , }) ; diag . code (E0207) ; res = Err (diag . emit ()) ; } } ty :: GenericParamDefKind :: Type { .. } | ty :: GenericParamDefKind :: Const { .. } => { } } } res }
}

macro_rules! enforce_impl_non_lifetime_params_are_constrained_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enforce_impl_non_lifetime_params_are_constrained in module {}", module_path!());
    };
}

mkfn!{
    enforce_impl_non_lifetime_params_are_constrained_introspect!();
    pub (crate) fn enforce_impl_non_lifetime_params_are_constrained (tcx : TyCtxt < '_ > , impl_def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { let impl_self_ty = tcx . type_of (impl_def_id) . instantiate_identity () ; if impl_self_ty . references_error () { tcx . dcx () . span_delayed_bug (tcx . def_span (impl_def_id) , format ! ("potentially unconstrained type parameters weren't evaluated: {impl_self_ty:?}" ,) ,) ; return Ok (()) ; } let impl_generics = tcx . generics_of (impl_def_id) ; let impl_predicates = tcx . predicates_of (impl_def_id) ; let impl_trait_ref = tcx . impl_trait_ref (impl_def_id) . map (ty :: EarlyBinder :: instantiate_identity) ; impl_trait_ref . error_reported () ? ; let mut input_parameters = cgp :: parameters_for_impl (tcx , impl_self_ty , impl_trait_ref) ; cgp :: identify_constrained_generic_params (tcx , impl_predicates , impl_trait_ref , & mut input_parameters ,) ; let mut res = Ok (()) ; for param in & impl_generics . own_params { let err = match param . kind { ty :: GenericParamDefKind :: Type { .. } => { let param_ty = ty :: ParamTy :: for_def (param) ; ! input_parameters . contains (& cgp :: Parameter :: from (param_ty)) } ty :: GenericParamDefKind :: Const { .. } => { let param_ct = ty :: ParamConst :: for_def (param) ; ! input_parameters . contains (& cgp :: Parameter :: from (param_ct)) } ty :: GenericParamDefKind :: Lifetime => { false } } ; if err { let const_param_note = matches ! (param . kind , ty :: GenericParamDefKind :: Const { .. }) ; let mut diag = tcx . dcx () . create_err (UnconstrainedGenericParameter { span : tcx . def_span (param . def_id) , param_name : tcx . item_ident (param . def_id) , param_def_kind : tcx . def_descr (param . def_id) , const_param_note , const_param_note2 : const_param_note , }) ; diag . code (E0207) ; res = Err (diag . emit ()) ; } } res }
}