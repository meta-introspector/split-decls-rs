macro_rules! deps {
    () => {
        CfgAtom!();
    };
}

macro_rules! CfgExpr {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] # [cfg_attr (test , derive (arbitrary :: Arbitrary))] pub enum CfgExpr { Invalid , Atom (CfgAtom) , All (Box < [CfgExpr] >) , Any (Box < [CfgExpr] >) , Not (Box < CfgExpr >) , }
    };
}

CfgExpr!()