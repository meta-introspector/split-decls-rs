macro_rules! deps {
    () => {
        Path!();
        Walkable!();
        Block!();
        QSelf!();
    };
}

macro_rules! DelegationMac {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct DelegationMac { pub qself : Option < Box < QSelf > > , pub prefix : Path , pub suffixes : Option < ThinVec < (Ident , Option < Ident >) > > , pub body : Option < Box < Block > > , }
    };
}

DelegationMac!()