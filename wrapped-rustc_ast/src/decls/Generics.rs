macro_rules! deps {
    () => {
        WhereClause!();
        Walkable!();
        GenericParam!();
    };
}

macro_rules! Generics {
    () => {
        deps!();
        # [doc = " Represents lifetime, type and const parameters attached to a declaration of"] # [doc = " a function, enum, trait, etc."] # [derive (Clone , Encodable , Decodable , Debug , Default , Walkable)] pub struct Generics { pub params : ThinVec < GenericParam > , pub where_clause : WhereClause , pub span : Span , }
    };
}

Generics!();