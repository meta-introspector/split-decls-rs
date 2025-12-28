macro_rules! UintSize {
    () => {
        # [cfg (not (feature = "mpmc_large"))] type UintSize = u8 ;
    };
}

UintSize!();