macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! Const {
    () => {
        deps!();
        # [derive (Copy , Clone , PartialEq , Eq , Hash , Encodable , Decodable , Debug)] # [derive (HashStable_Generic , Walkable)] pub enum Const { Yes (Span) , No , }
    };
}

Const!();