macro_rules! NoteCallerChoosesTyForTyParam {
    () => {
        # [derive (Subdiagnostic)] # [note (hir_typeck_note_caller_chooses_ty_for_ty_param)] pub (crate) struct NoteCallerChoosesTyForTyParam < 'tcx > { pub ty_param_name : Symbol , pub found_ty : Ty < 'tcx > , }
    };
}

NoteCallerChoosesTyForTyParam!();