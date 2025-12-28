macro_rules! NonUpperCaseGlobalSubTool {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (lint_suggestion , code = "{replace}" , applicability = "machine-applicable" , style = "tool-only")] pub (crate) struct NonUpperCaseGlobalSubTool { # [primary_span] pub (crate) span : Span , pub (crate) replace : String , }
    };
}

NonUpperCaseGlobalSubTool!();