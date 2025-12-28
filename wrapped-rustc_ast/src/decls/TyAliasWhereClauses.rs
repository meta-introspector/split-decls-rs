macro_rules! deps {
    () => {
        TyAlias!();
        Walkable!();
        TyAliasWhereClause!();
    };
}

macro_rules! TyAliasWhereClauses {
    () => {
        deps!();
        # [doc = " The span information for the two where clauses on a `TyAlias`."] # [derive (Copy , Clone , Encodable , Decodable , Debug , Default , Walkable)] pub struct TyAliasWhereClauses { # [doc = " Before the equals sign."] pub before : TyAliasWhereClause , # [doc = " After the equals sign."] pub after : TyAliasWhereClause , # [doc = " The index in `TyAlias.generics.where_clause.predicates` that would split"] # [doc = " into predicates from the where clause before the equals sign and the ones"] # [doc = " from the where clause after the equals sign."] pub split : usize , }
    };
}

TyAliasWhereClauses!();