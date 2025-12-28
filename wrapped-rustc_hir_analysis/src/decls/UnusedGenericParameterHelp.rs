macro_rules! UnusedGenericParameterHelp {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum UnusedGenericParameterHelp { # [help (hir_analysis_unused_generic_parameter_adt_help)] Adt { param_name : Ident , phantom_data : String } , # [help (hir_analysis_unused_generic_parameter_adt_no_phantom_data_help)] AdtNoPhantomData { param_name : Ident } , # [help (hir_analysis_unused_generic_parameter_ty_alias_help)] TyAlias { param_name : Ident } , }
    };
}

UnusedGenericParameterHelp!()