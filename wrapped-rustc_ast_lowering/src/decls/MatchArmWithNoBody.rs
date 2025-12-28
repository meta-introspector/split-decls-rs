macro_rules! MatchArmWithNoBody {
    () => {
        # [derive (Diagnostic)] # [diag (ast_lowering_match_arm_with_no_body)] pub (crate) struct MatchArmWithNoBody { # [primary_span] pub span : Span , # [suggestion (code = " => todo!()," , applicability = "has-placeholders")] pub suggestion : Span , }
    };
}

MatchArmWithNoBody!();