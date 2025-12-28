macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! TypeName {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Clone)] pub struct TypeName { pub namespace : String , pub name : String , pub generics : Vec < Type > , }
    };
}

TypeName!();