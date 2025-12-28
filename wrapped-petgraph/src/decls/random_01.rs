macro_rules! random_01 {
    () => {
        # [doc = " Return a random float in the range [0, 1.)"] fn random_01 < G : Gen > (g : & mut G) -> f64 { let bits = 53 ; let scale = 1. / ((1u64 << bits) as f64) ; let x : u64 = g . next_u64 () ; (x >> (64 - bits)) as f64 * scale }
    };
}

random_01!()