macro_rules! deps {
    () => {
        Type!();
        NewTypesKey!();
        AlternativeExprs!();
        ScopeDef!();
    };
}

macro_rules! LookupTable {
    () => {
        deps!();
        # [doc = " # Lookup table for term search"] # [doc = ""] # [doc = " Lookup table keeps all the state during term search."] # [doc = " This means it knows what types and how are reachable."] # [doc = ""] # [doc = " The secondary functionality for lookup table is to keep track of new types reached since last"] # [doc = " iteration as well as keeping track of which `ScopeDef` items have been used."] # [doc = " Both of them are to speed up the term search by leaving out types / ScopeDefs that likely do"] # [doc = " not produce any new results."] # [derive (Default , Debug)] struct LookupTable < 'db > { # [doc = " All the `Expr`s in \"value\" produce the type of \"key\""] data : FxHashMap < Type < 'db > , AlternativeExprs < 'db > > , # [doc = " New types reached since last query by the `NewTypesKey`"] new_types : FxHashMap < NewTypesKey , Vec < Type < 'db > > > , # [doc = " Types queried but not present"] types_wishlist : FxHashSet < Type < 'db > > , # [doc = " Threshold to squash trees to `Many`"] many_threshold : usize , }
    };
}

LookupTable!()