macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! FailedToGetLayout {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_failed_to_get_layout)] pub struct FailedToGetLayout < 'tcx > { # [primary_span] pub span : Span , pub ty : Ty < 'tcx > , pub err : LayoutError < 'tcx > , }
    };
}

FailedToGetLayout!()