macro_rules! deps {
    () => {
        Regex!();
    };
}

macro_rules! HybridEngine {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct HybridEngine (# [cfg (feature = "hybrid")] hybrid :: regex :: Regex , # [cfg (not (feature = "hybrid"))] () ,) ;
    };
}

HybridEngine!();