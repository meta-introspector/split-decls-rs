macro_rules! deps {
    () => {
        Const!();
        Ty!();
        Walkable!();
        AnonConst!();
    };
}

macro_rules! Term {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum Term { Ty (Box < Ty >) , Const (AnonConst) , }
    };
}

Term!();