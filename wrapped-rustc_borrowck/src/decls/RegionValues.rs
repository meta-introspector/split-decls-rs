macro_rules! deps {
    () => {
        PlaceholderIndices!();
        LivenessValues!();
    };
}

macro_rules! RegionValues {
    () => {
        deps!();
        # [doc = " Stores the full values for a set of regions (in contrast to"] # [doc = " `LivenessValues`, which only stores those points in the where a"] # [doc = " region is live). The full value for a region may contain points in"] # [doc = " the CFG, but also free regions as well as bound universe"] # [doc = " placeholders."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```text"] # [doc = " fn foo(x: &'a u32) -> &'a u32 {"] # [doc = "    let y: &'0 u32 = x; // let's call this `'0`"] # [doc = "    y"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Here, the variable `'0` would contain the free region `'a`,"] # [doc = " because (since it is returned) it must live for at least `'a`. But"] # [doc = " it would also contain various points from within the function."] pub (crate) struct RegionValues < N : Idx > { location_map : Rc < DenseLocationMap > , placeholder_indices : PlaceholderIndices , points : SparseIntervalMatrix < N , PointIndex > , free_regions : SparseBitMatrix < N , RegionVid > , # [doc = " Placeholders represent bound regions -- so something like `'a`"] # [doc = " in `for<'a> fn(&'a u32)`."] placeholders : SparseBitMatrix < N , PlaceholderIndex > , }
    };
}

RegionValues!();