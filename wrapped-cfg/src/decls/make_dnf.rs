macro_rules! deps {
    () => {
        CfgExpr!();
    };
}

macro_rules! make_dnf {
    () => {
        deps!();
        fn make_dnf (expr : CfgExpr) -> CfgExpr { match expr { CfgExpr :: Invalid | CfgExpr :: Atom (_) | CfgExpr :: Not (_) => expr , CfgExpr :: Any (e) => flatten (CfgExpr :: Any (e . into_vec () . into_iter () . map (make_dnf) . collect ())) , CfgExpr :: All (e) => { let e = e . into_vec () . into_iter () . map (make_dnf) . collect :: < Vec < _ > > () ; flatten (CfgExpr :: Any (distribute_conj (& e) . into_boxed_slice ())) } } }
    };
}

make_dnf!()