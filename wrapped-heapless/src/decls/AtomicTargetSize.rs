macro_rules! AtomicTargetSize {
    () => {
        # [cfg (not (feature = "mpmc_large"))] type AtomicTargetSize = atomic :: AtomicU8 ;
    };
}

AtomicTargetSize!()