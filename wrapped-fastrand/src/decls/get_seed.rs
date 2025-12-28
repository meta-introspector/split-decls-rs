macro_rules! get_seed {
    () => {
        # [doc = " Gives back **current** seed that is being held by the thread-local generator."] # [inline] pub fn get_seed () -> u64 { with_rng (| r | r . get_seed ()) }
    };
}

get_seed!()