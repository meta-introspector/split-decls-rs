macro_rules! deps {
    () => {
        Subdiagnostic!();
    };
}

macro_rules! MissingFeatures {
    () => {
        deps!();
        # [derive (Subdiagnostic)] # [help (codegen_ssa_missing_features)] pub struct MissingFeatures ;
    };
}

MissingFeatures!();