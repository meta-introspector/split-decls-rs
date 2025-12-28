macro_rules! deps {
    () => {
        Substitutable!();
    };
}

macro_rules! SubstitutionTable {
    () => {
        deps!();
        # [doc = " The table of substitutable components that we have parsed thus far, and for"] # [doc = " which there are potential back-references."] # [doc (hidden)] # [derive (Clone , Default , PartialEq , Eq)] pub struct SubstitutionTable { substitutions : Vec < Substitutable > , non_substitutions : Vec < Substitutable > , }
    };
}

SubstitutionTable!()