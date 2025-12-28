macro_rules! deps {
    () => {
        IntegerToPtrTransmutesSuggestion!();
    };
}

macro_rules! IntegerToPtrTransmutes {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (lint_int_to_ptr_transmutes)] # [note] # [note (lint_note_exposed_provenance)] # [help (lint_suggestion_without_provenance_mut)] # [help (lint_help_transmute)] # [help (lint_help_exposed_provenance)] pub (crate) struct IntegerToPtrTransmutes < 'tcx > { # [subdiagnostic] pub suggestion : Option < IntegerToPtrTransmutesSuggestion < 'tcx > > , }
    };
}

IntegerToPtrTransmutes!();