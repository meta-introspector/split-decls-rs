macro_rules! OnlyCurrentTraits {
    () => {
        # [derive (Diagnostic)] pub (crate) enum OnlyCurrentTraits { # [diag (hir_analysis_only_current_traits_outside , code = E0117)] Outside { # [primary_span] span : Span , # [note (hir_analysis_only_current_traits_note_uncovered)] # [note (hir_analysis_only_current_traits_note_more_info)] # [note (hir_analysis_only_current_traits_note)] note : () , } , # [diag (hir_analysis_only_current_traits_primitive , code = E0117)] Primitive { # [primary_span] span : Span , # [note (hir_analysis_only_current_traits_note_uncovered)] # [note (hir_analysis_only_current_traits_note_more_info)] # [note (hir_analysis_only_current_traits_note)] note : () , } , # [diag (hir_analysis_only_current_traits_arbitrary , code = E0117)] Arbitrary { # [primary_span] span : Span , # [note (hir_analysis_only_current_traits_note_uncovered)] # [note (hir_analysis_only_current_traits_note_more_info)] # [note (hir_analysis_only_current_traits_note)] note : () , } , }
    };
}

OnlyCurrentTraits!()