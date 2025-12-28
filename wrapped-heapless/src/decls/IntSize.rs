macro_rules! IntSize {
    () => {
        # [cfg (not (feature = "mpmc_large"))] type IntSize = i8 ;
    };
}

IntSize!();