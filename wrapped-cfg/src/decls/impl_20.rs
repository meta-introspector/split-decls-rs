macro_rules! deps {
    () => {
        Conjunction!();
        DnfExpr!();
        CfgExpr!();
        Builder!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Builder { fn lower (mut self , expr : & CfgExpr) -> DnfExpr { let expr = make_nnf (expr) ; let expr = make_dnf (expr) ; match expr { CfgExpr :: Invalid | CfgExpr :: Atom (_) | CfgExpr :: Not (_) => { self . expr . conjunctions . push (Conjunction :: new (Box :: new ([expr]))) ; } CfgExpr :: All (conj) => { self . expr . conjunctions . push (Conjunction :: new (conj)) ; } CfgExpr :: Any (disj) => { let mut disj = disj . into_vec () ; disj . reverse () ; while let Some (conj) = disj . pop () { match conj { CfgExpr :: Invalid | CfgExpr :: Atom (_) | CfgExpr :: All (_) | CfgExpr :: Not (_) => { self . expr . conjunctions . push (Conjunction :: new (Box :: new ([conj]))) ; } CfgExpr :: Any (inner_disj) => { disj . extend (inner_disj . into_vec () . into_iter () . rev ()) ; } } } } } self . expr } }
    };
}

impl_20!()