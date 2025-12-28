macro_rules! CfgDiff {
    () => {
        # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct CfgDiff { enable : Vec < CfgAtom > , disable : Vec < CfgAtom > , }
    };
}

CfgDiff!()