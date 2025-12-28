macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! ReverseHybridCache {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub (crate) struct ReverseHybridCache (# [cfg (feature = "hybrid")] Option < hybrid :: dfa :: Cache > , # [cfg (not (feature = "hybrid"))] () ,) ;
    };
}

ReverseHybridCache!()