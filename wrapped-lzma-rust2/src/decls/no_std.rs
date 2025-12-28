macro_rules! no_std {
    () => {
        # [cfg (not (feature = "std"))] mod no_std ;
    };
}

no_std!();