macro_rules! deps {
    () => {
        TypeName!();
        Type!();
    };
}

macro_rules! Remap {
    () => {
        deps!();
        # [derive (PartialEq)] pub enum Remap { Type (Type) , Name (TypeName) , None , }
    };
}

Remap!()