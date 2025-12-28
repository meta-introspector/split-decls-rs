macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! new_rng {
    () => {
        deps!();
        pub fn new_rng () -> Rng { SEED_RAND . with (| r | { let mut r = r . borrow_mut () ; let seed = ((r . rand_u64 () as u128) << 64) | (r . rand_u64 () as u128) ; Rand64 :: new (seed) }) }
    };
}

new_rng!()