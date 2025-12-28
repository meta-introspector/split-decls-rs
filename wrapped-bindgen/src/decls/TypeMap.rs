macro_rules! deps {
    () => {
        TypeName!();
        Type!();
    };
}

macro_rules! TypeMap {
    () => {
        deps!();
        # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct TypeMap (HashMap < TypeName , HashSet < Type > >) ;
    };
}

TypeMap!();