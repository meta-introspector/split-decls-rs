macro_rules! deps {
    () => {
        Path!();
        QPath!();
        Expr!();
        ExprKind!();
    };
}

macro_rules! is_range_literal {
    () => {
        deps!();
        # [doc = " Checks if the specified expression is a built-in range literal."] # [doc = " (See: `LoweringContext::lower_expr()`)."] pub fn is_range_literal (expr : & Expr < '_ >) -> bool { match expr . kind { ExprKind :: Struct (ref qpath , _ , _) => matches ! (** qpath , QPath :: LangItem (LangItem :: Range | LangItem :: RangeTo | LangItem :: RangeFrom | LangItem :: RangeFull | LangItem :: RangeToInclusive | LangItem :: RangeCopy | LangItem :: RangeFromCopy | LangItem :: RangeInclusiveCopy | LangItem :: RangeToInclusiveCopy , ..)) , ExprKind :: Call (ref func , _) => { matches ! (func . kind , ExprKind :: Path (QPath :: LangItem (LangItem :: RangeInclusiveNew , ..))) } _ => false , } }
    };
}

is_range_literal!()