macro_rules! deps {
    () => {
        UnexpectedExprKind!();
        ExprToSpannedString!();
    };
}

macro_rules! ExprToSpannedStringResult {
    () => {
        deps!();
        # [doc = " `Ok` represents successfully retrieving the string literal at the correct"] # [doc = " position, e.g., `println(\"abc\")`."] pub (crate) type ExprToSpannedStringResult < 'a > = Result < ExprToSpannedString , UnexpectedExprKind < 'a > > ;
    };
}

ExprToSpannedStringResult!();