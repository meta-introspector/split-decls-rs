macro_rules! deps {
    () => {
        FnMutReturnTypeErr!();
    };
}

macro_rules! FnMutError {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (borrowck_var_cannot_escape_closure)] # [note] # [note (borrowck_cannot_escape)] pub (crate) struct FnMutError { # [primary_span] pub span : Span , # [subdiagnostic] pub ty_err : FnMutReturnTypeErr , }
    };
}

FnMutError!();