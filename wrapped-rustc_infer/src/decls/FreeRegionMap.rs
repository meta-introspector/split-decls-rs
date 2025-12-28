macro_rules! FreeRegionMap {
    () => {
        # [derive (Clone , Debug)] pub struct FreeRegionMap < 'tcx > { # [doc = " Stores the relation `a < b`, where `a` and `b` are regions."] # [doc = ""] # [doc = " Invariant: only free regions like `'x` or `'static` are stored"] # [doc = " in this relation, not scopes."] pub (crate) relation : TransitiveRelation < Region < 'tcx > > , }
    };
}

FreeRegionMap!()