macro_rules! deps {
    () => {
        CfgExpr!();
    };
}

macro_rules! make_nnf {
    () => {
        deps!();
        fn make_nnf (expr : & CfgExpr) -> CfgExpr { match expr { CfgExpr :: Invalid | CfgExpr :: Atom (_) => expr . clone () , CfgExpr :: Any (expr) => CfgExpr :: Any (expr . iter () . map (make_nnf) . collect ()) , CfgExpr :: All (expr) => CfgExpr :: All (expr . iter () . map (make_nnf) . collect ()) , CfgExpr :: Not (operand) => make_nnf_neg (operand) , } }
    };
}

make_nnf!();