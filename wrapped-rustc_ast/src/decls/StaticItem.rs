macro_rules! deps {
    () => {
        Safety!();
        Ty!();
        Path!();
        Walkable!();
        Expr!();
    };
}

macro_rules! StaticItem {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct StaticItem { pub ident : Ident , pub ty : Box < Ty > , pub safety : Safety , pub mutability : Mutability , pub expr : Option < Box < Expr > > , pub define_opaque : Option < ThinVec < (NodeId , Path) > > , }
    };
}

StaticItem!();