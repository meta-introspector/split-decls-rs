macro_rules! PassToVariadicFunction {
    () => {
        # [derive (Diagnostic)] # [diag (hir_typeck_pass_to_variadic_function , code = E0617)] pub (crate) struct PassToVariadicFunction < 'a , 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub cast_ty : & 'a str , # [suggestion (code = " as {cast_ty}" , applicability = "machine-applicable" , style = "verbose")] pub sugg_span : Span , # [note (hir_typeck_teach_help)] pub (crate) teach : bool , }
    };
}

PassToVariadicFunction!();