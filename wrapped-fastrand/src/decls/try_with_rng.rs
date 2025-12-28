macro_rules! deps {
    () => {
        RestoreOnDrop!();
        Rng!();
    };
}

macro_rules! try_with_rng {
    () => {
        deps!();
        # [doc = " Try to run an operation with the current thread-local generator."] # [inline] fn try_with_rng < R > (f : impl FnOnce (& mut Rng) -> R) -> Result < R , std :: thread :: AccessError > { RNG . try_with (| rng | { let current = rng . replace (Rng (0)) ; let mut restore = RestoreOnDrop { rng , current } ; f (& mut restore . current) }) }
    };
}

try_with_rng!()