macro_rules! seed {
    () => {
        # [doc = " Initializes the thread-local generator with the given seed."] # [inline] pub fn seed (seed : u64) { with_rng (| r | r . seed (seed)) ; }
    };
}

seed!();