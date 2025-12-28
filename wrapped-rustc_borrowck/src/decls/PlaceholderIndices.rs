macro_rules! PlaceholderIndices {
    () => {
        # [doc = " Maps from `ty::PlaceholderRegion` values that are used in the rest of"] # [doc = " rustc to the internal `PlaceholderIndex` values that are used in"] # [doc = " NLL."] # [derive (Debug , Default)] # [derive (Clone)] pub (crate) struct PlaceholderIndices { indices : FxIndexSet < ty :: PlaceholderRegion > , }
    };
}

PlaceholderIndices!();