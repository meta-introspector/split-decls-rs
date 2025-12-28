macro_rules! OptStage {
    () => {
        # [doc = " LLVMRustOptStage"] # [derive (PartialEq)] # [repr (C)] pub (crate) enum OptStage { PreLinkNoLTO , PreLinkThinLTO , PreLinkFatLTO , ThinLTO , FatLTO , }
    };
}

OptStage!()