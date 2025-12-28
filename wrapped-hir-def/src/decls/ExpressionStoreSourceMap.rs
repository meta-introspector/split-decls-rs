macro_rules! deps {
    () => {
        ExpressionOnlySourceMap!();
        TypeSource!();
        LifetimeSource!();
    };
}

macro_rules! ExpressionStoreSourceMap {
    () => {
        deps!();
        # [derive (Debug , Eq , Default)] pub struct ExpressionStoreSourceMap { expr_only : Option < Box < ExpressionOnlySourceMap > > , types_map_back : ArenaMap < TypeRefId , TypeSource > , types_map : FxHashMap < TypeSource , TypeRefId > , lifetime_map_back : ArenaMap < LifetimeRefId , LifetimeSource > , # [expect (unused , reason = "this is here for completeness, and maybe we'll need it in the future")] lifetime_map : FxHashMap < LifetimeSource , LifetimeRefId > , }
    };
}

ExpressionStoreSourceMap!();