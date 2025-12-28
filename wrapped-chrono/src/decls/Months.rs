macro_rules! Months {
    () => {
        # [doc = " A duration in calendar months"] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq , PartialOrd , Ord)] # [cfg_attr (all (feature = "arbitrary" , feature = "std") , derive (arbitrary :: Arbitrary))] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub struct Months (pub (crate) u32) ;
    };
}

Months!()