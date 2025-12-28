macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! Delegate {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq , Ord , PartialOrd , Hash)] pub struct Delegate { pub def : TypeDef , pub generics : Vec < Type > , }
    };
}

Delegate!();