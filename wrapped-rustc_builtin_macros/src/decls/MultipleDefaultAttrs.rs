macro_rules! deps {
    () => {
        MultipleDefaultAttrsSugg!();
    };
}

macro_rules! MultipleDefaultAttrs {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (builtin_macros_multiple_default_attrs)] # [note] pub (crate) struct MultipleDefaultAttrs { # [primary_span] pub (crate) span : Span , # [label] pub (crate) first : Span , # [label (builtin_macros_label_again)] pub (crate) first_rest : Span , # [help] pub (crate) rest : MultiSpan , pub (crate) only_one : bool , # [subdiagnostic] pub (crate) sugg : MultipleDefaultAttrsSugg , }
    };
}

MultipleDefaultAttrs!()