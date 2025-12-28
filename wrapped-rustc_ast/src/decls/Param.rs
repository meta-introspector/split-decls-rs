macro_rules! deps {
    () => {
        AttrVec!();
        Ty!();
        Pat!();
        Walkable!();
    };
}

macro_rules! Param {
    () => {
        deps!();
        # [doc = " A parameter in a function header."] # [doc = ""] # [doc = " E.g., `bar: usize` as in `fn foo(bar: usize)`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct Param { pub attrs : AttrVec , pub ty : Box < Ty > , pub pat : Box < Pat > , pub id : NodeId , pub span : Span , pub is_placeholder : bool , }
    };
}

Param!()