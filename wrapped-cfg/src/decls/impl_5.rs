macro_rules! deps {
    () => {
        CfgExpr!();
        CfgAtom!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl From < CfgAtom > for CfgExpr { fn from (atom : CfgAtom) -> Self { CfgExpr :: Atom (atom) } }
    };
}

impl_5!();