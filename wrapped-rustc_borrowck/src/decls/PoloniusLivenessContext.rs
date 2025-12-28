macro_rules! deps {
    () => {
        ConstraintDirection!();
    };
}

macro_rules! PoloniusLivenessContext {
    () => {
        deps!();
        # [doc = " This struct holds the liveness data created during MIR typeck, and which will be used later in"] # [doc = " the process, to compute the polonius localized constraints."] # [derive (Default)] pub (crate) struct PoloniusLivenessContext { # [doc = " The expected edge direction per live region: the kind of directed edge we'll create as"] # [doc = " liveness constraints depends on the variance of types with respect to each contained region."] live_region_variances : BTreeMap < RegionVid , ConstraintDirection > , # [doc = " The regions that outlive free regions are used to distinguish relevant live locals from"] # [doc = " boring locals. A boring local is one whose type contains only such regions. Polonius"] # [doc = " currently has more boring locals than NLLs so we record the latter to use in errors and"] # [doc = " diagnostics, to focus on the locals we consider relevant and match NLL diagnostics."] pub (crate) boring_nll_locals : FxHashSet < Local > , }
    };
}

PoloniusLivenessContext!()