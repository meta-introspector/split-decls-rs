macro_rules! deps {
    () => {
        VisibilityKind!();
        Walkable!();
        LazyAttrTokenStream!();
    };
}

macro_rules! Visibility {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Visibility { pub kind : VisibilityKind , pub span : Span , pub tokens : Option < LazyAttrTokenStream > , }
    };
}

Visibility!();