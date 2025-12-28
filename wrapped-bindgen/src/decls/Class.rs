macro_rules! Class {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq , Ord , PartialOrd , Hash)] pub struct Class { pub def : TypeDef , }
    };
}

Class!();