macro_rules! deps {
    () => {
        AsCoercionSite!();
    };
}

macro_rules! Expressions {
    () => {
        deps!();
        enum Expressions < 'tcx , 'exprs , E : AsCoercionSite > { Dynamic (Vec < & 'tcx hir :: Expr < 'tcx > >) , UpFront (& 'exprs [E]) , }
    };
}

Expressions!()