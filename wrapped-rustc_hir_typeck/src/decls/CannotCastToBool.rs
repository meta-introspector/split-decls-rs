macro_rules! deps {
    () => {
        CannotCastToBoolHelp!();
    };
}

macro_rules! CannotCastToBool {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (hir_typeck_cannot_cast_to_bool , code = E0054)] pub (crate) struct CannotCastToBool < 'tcx > { # [primary_span] pub span : Span , pub expr_ty : Ty < 'tcx > , # [subdiagnostic] pub help : CannotCastToBoolHelp , }
    };
}

CannotCastToBool!()