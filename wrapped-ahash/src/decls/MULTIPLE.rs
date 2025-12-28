macro_rules! MULTIPLE {
    () => {
        # [doc = "This constant comes from Kunth's prng (Empirically it works better than those from splitmix32)."] pub (crate) const MULTIPLE : u64 = 6364136223846793005 ;
    };
}

MULTIPLE!()