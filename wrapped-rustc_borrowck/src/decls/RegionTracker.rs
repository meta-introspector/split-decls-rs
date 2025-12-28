macro_rules! deps {
    () => {
        Representative!();
        PlaceholderReachability!();
    };
}

macro_rules! RegionTracker {
    () => {
        deps!();
        # [doc = " An annotation for region graph SCCs that tracks"] # [doc = " the values of its elements. This annotates a single SCC."] # [derive (Copy , Debug , Clone)] pub (crate) struct RegionTracker { reachable_placeholders : PlaceholderReachability , # [doc = " The largest universe nameable from this SCC."] # [doc = " It is the smallest nameable universes of all"] # [doc = " existential regions reachable from it. Small Rvids are preferred."] max_nameable_universe : (UniverseIndex , RegionVid) , # [doc = " The representative Region Variable Id for this SCC."] pub (crate) representative : Representative , }
    };
}

RegionTracker!()