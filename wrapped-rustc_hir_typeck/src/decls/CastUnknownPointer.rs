macro_rules! deps {
    () => {
        CastUnknownPointerSub!();
    };
}

macro_rules! CastUnknownPointer {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_typeck_cast_unknown_pointer , code = E0641)] pub (crate) struct CastUnknownPointer { # [primary_span] pub span : Span , pub to : bool , # [subdiagnostic] pub sub : CastUnknownPointerSub , }
    };
}

CastUnknownPointer!();