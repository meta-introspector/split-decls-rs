macro_rules! FULL_DISTANCES {
    () => {
        const FULL_DISTANCES : usize = 1 << (DIST_MODEL_END / 2) ;
    };
}

FULL_DISTANCES!();