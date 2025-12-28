macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! ShuffleIndicesEvaluation {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_shuffle_indices_evaluation)] pub (crate) struct ShuffleIndicesEvaluation { # [primary_span] pub span : Span , }
    };
}

ShuffleIndicesEvaluation!();