macro_rules! VarHereDenote {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum VarHereDenote { # [label (borrowck_var_here_captured)] Captured { # [primary_span] span : Span , } , # [label (borrowck_var_here_defined)] Defined { # [primary_span] span : Span , } , # [label (borrowck_closure_inferred_mut)] FnMutInferred { # [primary_span] span : Span , } , }
    };
}

VarHereDenote!();