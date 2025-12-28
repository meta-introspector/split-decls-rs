macro_rules! Metadata {
    () => {
        # [cfg (windows)] # [doc = " A structure to partially mirror [`std::fs::Metadata`]."] pub struct Metadata (std :: fs :: Metadata) ;
    };
}

Metadata!()