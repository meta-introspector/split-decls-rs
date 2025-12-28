macro_rules! ExprKind {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum ExprKind { Expr , Expr2021 , }
    };
}

ExprKind!()