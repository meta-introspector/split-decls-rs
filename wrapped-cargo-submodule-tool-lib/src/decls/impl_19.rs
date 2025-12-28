macro_rules! deps {
    () => {
        Layer0Analyzer!();
        RealLayer0Analyzer!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        # [cfg (not (feature = "nix_generation"))] impl Layer0Analyzer for RealLayer0Analyzer { fn find_most_used_layer0_module (& self , _merged_data : & HashMap < String , MergedCrateInfo > ,) -> Result < Option < (String , u32) > > { Err (anyhow ! ("`Layer0Analyzer` requires the `nix_generation` feature to be enabled.")) } }
    };
}

impl_19!();