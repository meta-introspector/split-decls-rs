// Generated macro for enforce_trait_manually_implementable (function)
macro_rules! Depcrate_coherenceenforce_trait_manually_implementable {
() => {
// Module: crate::coherence
// Provides: {"enforce_trait_manually_implementable"}
// Dependencies: {}
fn enforce_trait_manually_implementable (tcx : TyCtxt < '_ > , impl_def_id : LocalDefId , trait_def_id : DefId , trait_def : & ty :: TraitDef ,) -> Result < () , ErrorGuaranteed > { let impl_header_span = tcx . def_span (impl_def_id) ; if tcx . is_lang_item (trait_def_id , LangItem :: Freeze) && ! tcx . features () . freeze_impls () { feature_err (& tcx . sess , sym :: freeze_impls , impl_header_span , "explicit impls for the `Freeze` trait are not permitted" ,) . with_span_label (impl_header_span , format ! ("impl of `Freeze` not allowed")) . emit () ; } if trait_def . deny_explicit_impl { let trait_name = tcx . item_name (trait_def_id) ; let mut err = struct_span_code_err ! (tcx . dcx () , impl_header_span , E0322 , "explicit impls for the `{trait_name}` trait are not permitted") ; err . span_label (impl_header_span , format ! ("impl of `{trait_name}` not allowed")) ; if tcx . is_lang_item (trait_def_id , LangItem :: Unsize) { err . code (E0328) ; } return Err (err . emit ()) ; } if let ty :: trait_def :: TraitSpecializationKind :: AlwaysApplicable = trait_def . specialization_kind { if ! tcx . features () . specialization () && ! tcx . features () . min_specialization () && ! impl_header_span . allows_unstable (sym :: specialization) && ! impl_header_span . allows_unstable (sym :: min_specialization) { return Err (tcx . dcx () . emit_err (errors :: SpecializationTrait { span : impl_header_span })) ; } } Ok (()) }
};
}
