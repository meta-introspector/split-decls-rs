macro_rules! RealNonVendoredModuleFinder {
    () => {
        # [cfg (not (feature = "nix_generation"))] pub struct RealNonVendoredModuleFinder ;
    };
}

RealNonVendoredModuleFinder!();