macro_rules! deps {
    () => {
        Subdiagnostic!();
    };
}

macro_rules! PossibleFeature {
    () => {
        deps!();
        # [derive (Subdiagnostic)] pub (crate) enum PossibleFeature < 'a > { # [help (codegen_ssa_possible_feature)] Some { rust_feature : & 'a str } , # [help (codegen_ssa_consider_filing_feature_request)] None , }
    };
}

PossibleFeature!();