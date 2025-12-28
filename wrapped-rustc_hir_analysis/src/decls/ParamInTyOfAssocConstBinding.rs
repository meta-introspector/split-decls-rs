macro_rules! deps {
    () => {
        TyOfAssocConstBindingNote!();
    };
}

macro_rules! ParamInTyOfAssocConstBinding {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_analysis_param_in_ty_of_assoc_const_binding)] pub (crate) struct ParamInTyOfAssocConstBinding < 'tcx > { # [primary_span] # [label] pub span : Span , pub assoc_const : Ident , pub param_name : Symbol , pub param_def_kind : & 'static str , pub param_category : & 'static str , # [label (hir_analysis_param_defined_here_label)] pub param_defined_here_label : Option < Span > , # [subdiagnostic] pub ty_note : Option < TyOfAssocConstBindingNote < 'tcx > > , }
    };
}

ParamInTyOfAssocConstBinding!()