macro_rules! InvalidReceiverTyHint {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum InvalidReceiverTyHint { # [note (hir_analysis_invalid_receiver_ty_help_weak_note)] Weak , # [note (hir_analysis_invalid_receiver_ty_help_nonnull_note)] NonNull , }
    };
}

InvalidReceiverTyHint!()