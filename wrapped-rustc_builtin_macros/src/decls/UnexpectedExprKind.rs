macro_rules! UnexpectedExprKind {
    () => {
        # [doc = " - `Ok` is returned when the conversion to a string literal is unsuccessful,"] # [doc = " but another type of expression is obtained instead."] # [doc = " - `Err` is returned when the conversion process fails."] type UnexpectedExprKind < 'a > = Result < (Diag < 'a > , bool) , ErrorGuaranteed > ;
    };
}

UnexpectedExprKind!();