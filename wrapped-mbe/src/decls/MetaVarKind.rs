macro_rules! deps {
    () => {
        ExprKind!();
    };
}

macro_rules! MetaVarKind {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub (crate) enum MetaVarKind { Path , Ty , Pat , PatParam , Stmt , Block , Meta , Item , Vis , Expr (ExprKind) , Ident , Tt , Lifetime , Literal , }
    };
}

MetaVarKind!()