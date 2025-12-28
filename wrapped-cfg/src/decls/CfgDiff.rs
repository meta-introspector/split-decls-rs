macro_rules! deps {
    () => {
        CfgAtom!();
    };
}

macro_rules! CfgDiff {
    () => {
        deps!();
        # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct CfgDiff { enable : Vec < CfgAtom > , disable : Vec < CfgAtom > , }
    };
}

CfgDiff!()