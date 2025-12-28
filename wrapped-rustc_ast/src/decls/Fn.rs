macro_rules! deps {
    () => {
        Block!();
        Path!();
        Generics!();
        Defaultness!();
        FnContract!();
        FnSig!();
    };
}

macro_rules! Fn {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug)] pub struct Fn { pub defaultness : Defaultness , pub ident : Ident , pub generics : Generics , pub sig : FnSig , pub contract : Option < Box < FnContract > > , pub define_opaque : Option < ThinVec < (NodeId , Path) > > , pub body : Option < Box < Block > > , }
    };
}

Fn!()