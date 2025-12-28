macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! TypeTree {
    () => {
        deps!();
        # [derive (Clone , Eq , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub struct TypeTree (pub Vec < Type >) ;
    };
}

TypeTree!()