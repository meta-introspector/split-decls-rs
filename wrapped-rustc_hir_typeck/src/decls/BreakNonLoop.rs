macro_rules! BreakNonLoop {
    () => {
        pub (crate) struct BreakNonLoop < 'a > { pub span : Span , pub head : Option < Span > , pub kind : & 'a str , pub suggestion : String , pub loop_label : Option < Label > , pub break_label : Option < Label > , pub break_expr_kind : & 'a ExprKind < 'a > , pub break_expr_span : Span , }
    };
}

BreakNonLoop!();