macro_rules! RegionElement {
    () => {
        # [doc = " An individual element in a region value -- the value of a"] # [doc = " particular region variable consists of a set of these elements."] # [derive (Debug , Clone , PartialEq)] pub (crate) enum RegionElement { # [doc = " A point in the control-flow graph."] Location (Location) , # [doc = " A universally quantified region from the root universe (e.g.,"] # [doc = " a lifetime parameter)."] RootUniversalRegion (RegionVid) , # [doc = " A placeholder (e.g., instantiated from a `for<'a> fn(&'a u32)`"] # [doc = " type)."] PlaceholderRegion (ty :: PlaceholderRegion) , }
    };
}

RegionElement!();