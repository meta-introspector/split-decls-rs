macro_rules! deps {
    () => {
        Checker!();
    };
}

macro_rules! check_trait {
    () => {
        deps!();
        pub (super) fn check_trait < 'tcx > (tcx : TyCtxt < 'tcx > , trait_def_id : DefId , impl_def_id : LocalDefId , impl_header : ty :: ImplTraitHeader < 'tcx > ,) -> Result < () , ErrorGuaranteed > { let lang_items = tcx . lang_items () ; let checker = Checker { tcx , trait_def_id , impl_def_id , impl_header } ; checker . check (lang_items . drop_trait () , visit_implementation_of_drop) ? ; checker . check (lang_items . async_drop_trait () , visit_implementation_of_drop) ? ; checker . check (lang_items . copy_trait () , visit_implementation_of_copy) ? ; checker . check (lang_items . const_param_ty_trait () , | checker | { visit_implementation_of_const_param_ty (checker , LangItem :: ConstParamTy) }) ? ; checker . check (lang_items . unsized_const_param_ty_trait () , | checker | { visit_implementation_of_const_param_ty (checker , LangItem :: UnsizedConstParamTy) }) ? ; checker . check (lang_items . coerce_unsized_trait () , visit_implementation_of_coerce_unsized) ? ; checker . check (lang_items . dispatch_from_dyn_trait () , visit_implementation_of_dispatch_from_dyn) ? ; checker . check (lang_items . coerce_pointee_validated_trait () , visit_implementation_of_coerce_pointee_validity ,) ? ; Ok (()) }
    };
}

check_trait!()