macro_rules! Pad {
    () => {
        # [doc = " Padding characters for numeric items."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub enum Pad { # [doc = " No padding."] None , # [doc = " Zero (`0`) padding."] Zero , # [doc = " Space padding."] Space , }
    };
}

Pad!()