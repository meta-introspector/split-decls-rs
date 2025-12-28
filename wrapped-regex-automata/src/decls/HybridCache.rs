macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! HybridCache {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct HybridCache (# [cfg (feature = "hybrid")] Option < hybrid :: regex :: Cache > , # [cfg (not (feature = "hybrid"))] () ,) ;
    };
}

HybridCache!()