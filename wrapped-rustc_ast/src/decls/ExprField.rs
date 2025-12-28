macro_rules! deps {
    () => {
        Walkable!();
        Expr!();
        AttrVec!();
    };
}

macro_rules! ExprField {
    () => {
        deps!();
        # [doc = " A single field in a struct expression, e.g. `x: value` and `y` in `Foo { x: value, y }`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct ExprField { pub attrs : AttrVec , pub id : NodeId , pub span : Span , pub ident : Ident , pub expr : Box < Expr > , pub is_shorthand : bool , pub is_placeholder : bool , }
    };
}

ExprField!()