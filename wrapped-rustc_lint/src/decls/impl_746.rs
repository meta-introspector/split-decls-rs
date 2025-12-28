macro_rules! deps {
    () => {
        AmbiguousNegativeLiteralsCurrentBehaviorSuggestion!();
        AmbiguousNegativeLiteralsNegativeLiteralSuggestion!();
        EarlyContext!();
        AmbiguousNegativeLiteralsDiag!();
    };
}

macro_rules! impl_746 {
    () => {
        deps!();
        impl EarlyLintPass for Precedence { fn check_expr (& mut self , cx : & EarlyContext < '_ > , expr : & Expr) { let ExprKind :: Unary (UnOp :: Neg , operand) = & expr . kind else { return ; } ; let mut arg = operand ; let mut at_least_one = false ; while let ExprKind :: MethodCall (box MethodCall { receiver , .. }) = & arg . kind { at_least_one = true ; arg = receiver ; } if at_least_one && let ExprKind :: Lit (lit) = & arg . kind && let LitKind :: Integer | LitKind :: Float = & lit . kind { cx . emit_span_lint (AMBIGUOUS_NEGATIVE_LITERALS , expr . span , AmbiguousNegativeLiteralsDiag { negative_literal : AmbiguousNegativeLiteralsNegativeLiteralSuggestion { start_span : expr . span . shrink_to_lo () , end_span : arg . span . shrink_to_hi () , } , current_behavior : AmbiguousNegativeLiteralsCurrentBehaviorSuggestion { start_span : operand . span . shrink_to_lo () , end_span : operand . span . shrink_to_hi () , } , } ,) ; } } }
    };
}

impl_746!()