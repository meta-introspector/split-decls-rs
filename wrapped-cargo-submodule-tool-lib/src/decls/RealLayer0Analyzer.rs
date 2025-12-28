macro_rules! RealLayer0Analyzer {
    () => {
        # [cfg (not (feature = "nix_generation"))] pub struct RealLayer0Analyzer ;
    };
}

RealLayer0Analyzer!()