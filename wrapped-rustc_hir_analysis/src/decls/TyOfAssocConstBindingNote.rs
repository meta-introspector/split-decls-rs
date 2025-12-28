macro_rules! TyOfAssocConstBindingNote {
    () => {
        # [derive (Subdiagnostic , Clone , Copy)] # [note (hir_analysis_ty_of_assoc_const_binding_note)] pub (crate) struct TyOfAssocConstBindingNote < 'tcx > { pub assoc_const : Ident , pub ty : Ty < 'tcx > , }
    };
}

TyOfAssocConstBindingNote!()