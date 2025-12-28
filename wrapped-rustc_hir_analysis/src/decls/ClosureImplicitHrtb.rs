macro_rules! ClosureImplicitHrtb {
    () => {
        # [derive (Diagnostic)] # [diag (hir_analysis_closure_implicit_hrtb)] pub (crate) struct ClosureImplicitHrtb { # [primary_span] pub spans : Vec < Span > , # [label] pub for_sp : Span , }
    };
}

ClosureImplicitHrtb!()