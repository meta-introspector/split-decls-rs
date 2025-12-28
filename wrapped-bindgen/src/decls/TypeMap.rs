macro_rules! deps {
    () => {
        Type!();
        TypeName!();
    };
}

macro_rules! TypeMap {
    () => {
        deps!();
        # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct TypeMap (HashMap < TypeName , HashSet < Type > >) ;
    };
}

TypeMap!()