macro_rules! MisplacedImplTrait {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_misplaced_impl_trait , code = E0562)] # [note] pub (crate) struct MisplacedImplTrait < 'a > { # [primary_span] pub span : Span , pub position : DiagArgFromDisplay < 'a > , }
    };
}

MisplacedImplTrait!();