macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! UnstableReason {
    () => {
        deps!();
        # [derive (Encodable , Decodable , PartialEq , Copy , Clone , Debug , Eq , Hash)] # [derive (HashStable_Generic , PrintAttribute)] pub enum UnstableReason { None , Default , Some (Symbol) , }
    };
}

UnstableReason!()