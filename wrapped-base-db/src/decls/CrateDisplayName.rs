macro_rules! deps {
    () => {
        CrateName!();
    };
}

macro_rules! CrateDisplayName {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct CrateDisplayName { crate_name : CrateName , canonical_name : Symbol , }
    };
}

CrateDisplayName!()