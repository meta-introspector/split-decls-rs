macro_rules! deps {
    () => {
        CoverageSpan!();
        Counter!();
    };
}

macro_rules! BranchRegion {
    () => {
        deps!();
        # [doc = " Must match the layout of `LLVMRustCoverageBranchRegion`."] # [derive (Clone , Debug)] # [repr (C)] pub (crate) struct BranchRegion { pub (crate) cov_span : CoverageSpan , pub (crate) true_counter : Counter , pub (crate) false_counter : Counter , }
    };
}

BranchRegion!()