macro_rules! deps {
    () => {
        Generics!();
        Path!();
        Defaultness!();
        Ty!();
        Expr!();
        Walkable!();
    };
}

macro_rules! ConstItem {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct ConstItem { pub defaultness : Defaultness , pub ident : Ident , pub generics : Generics , pub ty : Box < Ty > , pub expr : Option < Box < Expr > > , pub define_opaque : Option < ThinVec < (NodeId , Path) > > , }
    };
}

ConstItem!();