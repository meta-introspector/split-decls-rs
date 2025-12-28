macro_rules! deps {
    () => {
        Test!();
        Seed!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl Test { fn new (seed : & Seed) -> Test { Test { haystack : seed . haystack . to_string () , needles : seed . needles . to_vec () , expected : seed . positions . to_vec () , } } }
    };
}

impl_25!()