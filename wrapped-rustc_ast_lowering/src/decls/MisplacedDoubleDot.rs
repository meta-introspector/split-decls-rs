macro_rules! MisplacedDoubleDot {
    () => {
        # [derive (Diagnostic)] # [note] # [diag (ast_lowering_misplaced_double_dot)] pub (crate) struct MisplacedDoubleDot { # [primary_span] pub span : Span , }
    };
}

MisplacedDoubleDot!();