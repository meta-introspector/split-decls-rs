macro_rules! deps {
    () => {
        LiveLoans!();
    };
}

macro_rules! LivenessValues {
    () => {
        deps!();
        # [doc = " Records the CFG locations where each region is live. When we initially compute liveness, we use"] # [doc = " an interval matrix storing liveness ranges for each region-vid."] # [derive (Clone)] pub (crate) struct LivenessValues { # [doc = " The map from locations to points."] location_map : Rc < DenseLocationMap > , # [doc = " Which regions are live. This is exclusive with the fine-grained tracking in `points`, and"] # [doc = " currently only used for validating promoteds (which don't care about more precise tracking)."] live_regions : Option < FxHashSet < RegionVid > > , # [doc = " For each region: the points where it is live."] # [doc = ""] # [doc = " This is not initialized for promoteds, because we don't care *where* within a promoted a"] # [doc = " region is live, only that it is."] points : Option < SparseIntervalMatrix < RegionVid , PointIndex > > , # [doc = " When using `-Zpolonius=next`, the set of loans that are live at a given point in the CFG."] live_loans : Option < LiveLoans > , }
    };
}

LivenessValues!();