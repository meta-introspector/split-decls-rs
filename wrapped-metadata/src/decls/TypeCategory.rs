macro_rules! deps {
    () => {
        Attribute!();
    };
}

macro_rules! TypeCategory {
    () => {
        deps!();
        # [derive (PartialEq)] pub enum TypeCategory { Interface , Class , Enum , Struct , Delegate , Attribute , }
    };
}

TypeCategory!()