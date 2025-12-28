macro_rules! deps {
    () => {
        CfgExpr!();
    };
}

macro_rules! make_nnf_neg {
    () => {
        deps!();
        fn make_nnf_neg (operand : & CfgExpr) -> CfgExpr { match operand { CfgExpr :: Invalid => CfgExpr :: Not (Box :: new (CfgExpr :: Invalid)) , CfgExpr :: Atom (atom) => CfgExpr :: Not (Box :: new (CfgExpr :: Atom (atom . clone ()))) , CfgExpr :: Not (expr) => make_nnf (expr) , CfgExpr :: Any (inner) => CfgExpr :: All (inner . iter () . map (make_nnf_neg) . collect ()) , CfgExpr :: All (inner) => CfgExpr :: Any (inner . iter () . map (make_nnf_neg) . collect ()) , } }
    };
}

make_nnf_neg!()