macro_rules! AddBound {
    () => {
        # [derive (Subdiagnostic)] # [suggestion (lint_opaque_hidden_inferred_bound_sugg , style = "verbose" , applicability = "machine-applicable" , code = " + {trait_ref}")] struct AddBound < 'tcx > { # [primary_span] suggest_span : Span , # [skip_arg] trait_ref : TraitPredPrintModifiersAndPath < 'tcx > , }
    };
}

AddBound!();