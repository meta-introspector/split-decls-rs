macro_rules! deps {
    () => {
        Walkable!();
        Generics!();
        Ty!();
        TyAliasWhereClauses!();
        BoundKind!();
        GenericBounds!();
        Defaultness!();
    };
}

macro_rules! TyAlias {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct TyAlias { pub defaultness : Defaultness , pub ident : Ident , pub generics : Generics , pub where_clauses : TyAliasWhereClauses , # [visitable (extra = BoundKind :: Bound)] pub bounds : GenericBounds , pub ty : Option < Box < Ty > > , }
    };
}

TyAlias!()