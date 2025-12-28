macro_rules! deps {
    () => {
        ChainFn!();
    };
}

macro_rules! chain_fn {
    () => {
        deps!();
        pub (crate) fn chain_fn < F , G > (f : F , g : G) -> ChainFn < F , G > { ChainFn (f , g) }
    };
}

chain_fn!();