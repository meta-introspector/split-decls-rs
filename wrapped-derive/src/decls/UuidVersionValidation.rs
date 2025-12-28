macro_rules! UuidVersionValidation {
    () => {
        # [derive (Clone , Debug , PartialEq)] pub enum UuidVersionValidation { None , Value (Lit) , }
    };
}

UuidVersionValidation!();