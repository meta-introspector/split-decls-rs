macro_rules! deps {
    () => {
        Walkable!();
    };
}

macro_rules! Lifetime {
    () => {
        deps!();
        # [doc = " A \"Lifetime\" is an annotation of the scope in which variable"] # [doc = " can be used, e.g. `'a` in `&'a i32`."] # [derive (Clone , Encodable , Decodable , Copy , PartialEq , Eq , Hash , Walkable)] pub struct Lifetime { pub id : NodeId , pub ident : Ident , }
    };
}

Lifetime!();