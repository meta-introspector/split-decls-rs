macro_rules! DiscriminantHints {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub enum DiscriminantHints { Always , Never , Fieldless , }
    };
}

DiscriminantHints!();