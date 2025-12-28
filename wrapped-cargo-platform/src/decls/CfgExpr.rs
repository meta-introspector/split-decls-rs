macro_rules! deps {
    () => {
        Cfg!();
    };
}

macro_rules! CfgExpr {
    () => {
        deps!();
        # [doc = " A cfg expression."] # [derive (Eq , PartialEq , Hash , Ord , PartialOrd , Clone , Debug)] pub enum CfgExpr { Not (Box < CfgExpr >) , All (Vec < CfgExpr >) , Any (Vec < CfgExpr >) , Value (Cfg) , True , False , }
    };
}

CfgExpr!()