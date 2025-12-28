macro_rules! deps {
    () => {
        Walkable!();
        Ty!();
    };
}

macro_rules! MutTy {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct MutTy { pub ty : Box < Ty > , pub mutbl : Mutability , }
    };
}

MutTy!();