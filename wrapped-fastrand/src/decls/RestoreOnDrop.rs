macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! RestoreOnDrop {
    () => {
        deps!();
        # [doc = " Make sure the original RNG is restored even on panic."] struct RestoreOnDrop < 'a > { rng : & 'a Cell < Rng > , current : Rng , }
    };
}

RestoreOnDrop!()