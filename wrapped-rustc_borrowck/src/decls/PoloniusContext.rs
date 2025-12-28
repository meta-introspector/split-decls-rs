macro_rules! deps {
    () => {
        PoloniusLivenessContext!();
    };
}

macro_rules! PoloniusContext {
    () => {
        deps!();
        # [doc = " This struct holds the data needed to create the Polonius localized constraints. Its data is"] # [doc = " transferred and converted from the [PoloniusLivenessContext] at the end of MIR typeck."] pub (crate) struct PoloniusContext { # [doc = " The liveness data we recorded during MIR typeck."] liveness_context : PoloniusLivenessContext , # [doc = " The set of regions that are live at a given point in the CFG, used to create localized"] # [doc = " outlives constraints between regions that are live at connected points in the CFG."] live_regions : SparseBitMatrix < PointIndex , RegionVid > , }
    };
}

PoloniusContext!()