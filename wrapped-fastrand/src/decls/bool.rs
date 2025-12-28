macro_rules! bool {
    () => {
        # [doc = " Generates a random `bool`."] # [inline] pub fn bool () -> bool { with_rng (| r | r . bool ()) }
    };
}

bool!();