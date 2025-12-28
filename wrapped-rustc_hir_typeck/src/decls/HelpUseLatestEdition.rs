macro_rules! HelpUseLatestEdition {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum HelpUseLatestEdition { # [help (hir_typeck_help_set_edition_cargo)] # [note (hir_typeck_note_edition_guide)] Cargo { edition : Edition } , # [help (hir_typeck_help_set_edition_standalone)] # [note (hir_typeck_note_edition_guide)] Standalone { edition : Edition } , }
    };
}

HelpUseLatestEdition!()