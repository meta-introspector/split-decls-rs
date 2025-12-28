macro_rules! deps {
    () => {
        CoverageSpan!();
        Counter!();
    };
}

macro_rules! CodeRegion {
    () => {
        deps!();
        # [doc = " Must match the layout of `LLVMRustCoverageCodeRegion`."] # [derive (Clone , Debug)] # [repr (C)] pub (crate) struct CodeRegion { pub (crate) cov_span : CoverageSpan , pub (crate) counter : Counter , }
    };
}

CodeRegion!()