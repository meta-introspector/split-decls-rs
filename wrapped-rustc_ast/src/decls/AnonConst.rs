macro_rules! deps {
    () => {
        Walkable!();
        Expr!();
    };
}

macro_rules! AnonConst {
    () => {
        deps!();
        # [doc = " A constant (expression) that's not an item or associated item,"] # [doc = " but needs its own `DefId` for type-checking, const-eval, etc."] # [doc = " These are usually found nested inside types (e.g., array lengths)"] # [doc = " or expressions (e.g., repeat counts), and also used to define"] # [doc = " explicit discriminant values for enum variants."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct AnonConst { pub id : NodeId , pub value : Box < Expr > , }
    };
}

AnonConst!()