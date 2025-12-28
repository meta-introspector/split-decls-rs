macro_rules! deps {
    () => {
        WherePredicate!();
        GenericParam!();
    };
}

macro_rules! Generics {
    () => {
        deps!();
        # [doc = " Represents lifetimes and type parameters attached to a declaration"] # [doc = " of a function, enum, trait, etc."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct Generics < 'hir > { pub params : & 'hir [GenericParam < 'hir >] , pub predicates : & 'hir [WherePredicate < 'hir >] , pub has_where_clause_predicates : bool , pub where_clause_span : Span , pub span : Span , }
    };
}

Generics!();