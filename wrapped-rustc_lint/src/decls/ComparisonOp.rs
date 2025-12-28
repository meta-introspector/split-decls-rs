macro_rules! ComparisonOp {
    () => {
        # [derive (Debug , PartialEq , Copy , Clone)] enum ComparisonOp { BinOp (hir :: BinOpKind) , Other , }
    };
}

ComparisonOp!();