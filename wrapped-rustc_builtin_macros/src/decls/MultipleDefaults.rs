macro_rules! deps {
    () => {
        MultipleDefaultsSugg!();
    };
}

macro_rules! MultipleDefaults {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (builtin_macros_multiple_defaults)] # [note] pub (crate) struct MultipleDefaults { # [primary_span] pub (crate) span : Span , # [label] pub (crate) first : Span , # [label (builtin_macros_additional)] pub additional : Vec < Span > , # [subdiagnostic] pub suggs : Vec < MultipleDefaultsSugg > , }
    };
}

MultipleDefaults!()