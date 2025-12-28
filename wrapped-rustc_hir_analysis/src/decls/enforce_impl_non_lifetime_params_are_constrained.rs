macro_rules! deps {
    () => {
        Parameter!();
        UnconstrainedGenericParameter!();
    };
}

macro_rules! enforce_impl_non_lifetime_params_are_constrained {
    () => {
        deps!();
        pub (crate) fn enforce_impl_non_lifetime_params_are_constrained (tcx : TyCtxt < '_ > , impl_def_id : LocalDefId ,) -> Result < () , ErrorGuaranteed > { let impl_self_ty = tcx . type_of (impl_def_id) . instantiate_identity () ; if impl_self_ty . references_error () { tcx . dcx () . span_delayed_bug (tcx . def_span (impl_def_id) , format ! ("potentially unconstrained type parameters weren't evaluated: {impl_self_ty:?}" ,) ,) ; return Ok (()) ; } let impl_generics = tcx . generics_of (impl_def_id) ; let impl_predicates = tcx . predicates_of (impl_def_id) ; let impl_trait_ref = tcx . impl_trait_ref (impl_def_id) . map (ty :: EarlyBinder :: instantiate_identity) ; impl_trait_ref . error_reported () ? ; let mut input_parameters = cgp :: parameters_for_impl (tcx , impl_self_ty , impl_trait_ref) ; cgp :: identify_constrained_generic_params (tcx , impl_predicates , impl_trait_ref , & mut input_parameters ,) ; let mut res = Ok (()) ; for param in & impl_generics . own_params { let err = match param . kind { ty :: GenericParamDefKind :: Type { .. } => { let param_ty = ty :: ParamTy :: for_def (param) ; ! input_parameters . contains (& cgp :: Parameter :: from (param_ty)) } ty :: GenericParamDefKind :: Const { .. } => { let param_ct = ty :: ParamConst :: for_def (param) ; ! input_parameters . contains (& cgp :: Parameter :: from (param_ct)) } ty :: GenericParamDefKind :: Lifetime => { false } } ; if err { let const_param_note = matches ! (param . kind , ty :: GenericParamDefKind :: Const { .. }) ; let mut diag = tcx . dcx () . create_err (UnconstrainedGenericParameter { span : tcx . def_span (param . def_id) , param_name : tcx . item_ident (param . def_id) , param_def_kind : tcx . def_descr (param . def_id) , const_param_note , const_param_note2 : const_param_note , }) ; diag . code (E0207) ; res = Err (diag . emit ()) ; } } res }
    };
}

enforce_impl_non_lifetime_params_are_constrained!();