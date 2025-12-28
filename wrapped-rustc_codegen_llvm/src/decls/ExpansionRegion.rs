macro_rules! deps {
    () => {
        CoverageSpan!();
    };
}

macro_rules! ExpansionRegion {
    () => {
        deps!();
        # [doc = " Must match the layout of `LLVMRustCoverageExpansionRegion`."] # [derive (Clone , Debug)] # [repr (C)] pub (crate) struct ExpansionRegion { pub (crate) cov_span : CoverageSpan , pub (crate) expanded_file_id : u32 , }
    };
}

ExpansionRegion!();