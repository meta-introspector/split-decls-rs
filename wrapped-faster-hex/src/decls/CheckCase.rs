macro_rules! CheckCase {
    () => {
        # [derive (Eq , PartialEq)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub enum CheckCase { None , Lower , Upper , }
    };
}

CheckCase!();