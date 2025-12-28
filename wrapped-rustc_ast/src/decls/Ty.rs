macro_rules! deps {
    () => {
        TyKind!();
        LazyAttrTokenStream!();
        Walkable!();
    };
}

macro_rules! Ty {
    () => {
        deps!();
        # [derive (Encodable , Decodable , Debug , Walkable)] pub struct Ty { pub id : NodeId , pub kind : TyKind , pub span : Span , pub tokens : Option < LazyAttrTokenStream > , }
    };
}

Ty!();