macro_rules! deps {
    () => {
        Struct!();
        Union!();
        Type!();
        Enum!();
    };
}

macro_rules! Adt {
    () => {
        deps!();
        # [doc = " A Data Type"] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub enum Adt { Struct (Struct) , Union (Union) , Enum (Enum) , }
    };
}

Adt!()