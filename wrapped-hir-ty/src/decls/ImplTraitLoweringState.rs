macro_rules! deps {
    () => {
        ImplTrait!();
        ImplTraitLoweringMode!();
    };
}

macro_rules! ImplTraitLoweringState {
    () => {
        deps!();
        # [derive (Debug , Default)] struct ImplTraitLoweringState < 'db > { # [doc = " When turning `impl Trait` into opaque types, we have to collect the"] # [doc = " bounds at the same time to get the IDs correct (without becoming too"] # [doc = " complicated)."] mode : ImplTraitLoweringMode , opaque_type_data : Arena < ImplTrait < 'db > > , }
    };
}

ImplTraitLoweringState!();