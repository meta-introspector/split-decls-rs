macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! IntType {
    () => {
        deps!();
        # [derive (Eq , PartialEq , Debug , Copy , Clone)] # [derive (Encodable , Decodable , HashStable_Generic , PrintAttribute)] pub enum IntType { SignedInt (ast :: IntTy) , UnsignedInt (ast :: UintTy) , }
    };
}

IntType!()