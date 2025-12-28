macro_rules! Error {
    () => {
        # [derive (Clone , Copy , PartialEq)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub enum Error { InvalidChar , InvalidLength (usize) , Overflow , }
    };
}

Error!()