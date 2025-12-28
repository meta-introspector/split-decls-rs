macro_rules! Wide {
    () => {
        # [cfg (fast_arithmetic = "64")] type Wide = u128 ;
    };
}

Wide!();