macro_rules! ImplicitUnsafeAutorefsMethodNote {
    () => {
        # [derive (Subdiagnostic)] # [note (lint_method_def)] pub (crate) struct ImplicitUnsafeAutorefsMethodNote { # [primary_span] pub def_span : Span , pub method_name : Symbol , }
    };
}

ImplicitUnsafeAutorefsMethodNote!();