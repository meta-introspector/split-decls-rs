macro_rules! deps {
    () => {
        Visibility!();
        Safety!();
        AnonConst!();
        Walkable!();
        AttrVec!();
        Ty!();
    };
}

macro_rules! FieldDef {
    () => {
        deps!();
        # [doc = " Field definition in a struct, variant or union."] # [doc = ""] # [doc = " E.g., `bar: usize` as in `struct Foo { bar: usize }`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct FieldDef { pub attrs : AttrVec , pub id : NodeId , pub span : Span , pub vis : Visibility , pub safety : Safety , pub ident : Option < Ident > , pub ty : Box < Ty > , pub default : Option < AnonConst > , pub is_placeholder : bool , }
    };
}

FieldDef!();