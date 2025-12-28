macro_rules! Layer0Analyzer {
    () => {
        # [cfg (not (feature = "nix_generation"))] pub trait Layer0Analyzer { fn find_most_used_layer0_module (& self , merged_data : & HashMap < String , MergedCrateInfo > ,) -> Result < Option < (String , u32) > > ; }
    };
}

Layer0Analyzer!();