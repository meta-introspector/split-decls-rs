macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! ByRef {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug , Eq , PartialEq)] # [derive (Encodable , Decodable , HashStable_Generic , Walkable)] pub enum ByRef { Yes (Mutability) , No , }
    };
}

ByRef!()