macro_rules! ImplicitUnsafeAutorefsOrigin {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum ImplicitUnsafeAutorefsOrigin < 'a > { # [note (lint_autoref)] Autoref { # [primary_span] autoref_span : Span , autoref_ty : Ty < 'a > , } , # [note (lint_overloaded_deref)] OverloadedDeref , }
    };
}

ImplicitUnsafeAutorefsOrigin!();