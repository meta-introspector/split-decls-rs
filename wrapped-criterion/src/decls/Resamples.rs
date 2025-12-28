macro_rules! deps {
    () => {
        Float!();
        Rng!();
    };
}

macro_rules! Resamples {
    () => {
        deps!();
        pub struct Resamples < 'a , A > where A : Float , { rng : Rng , sample : & 'a [A] , stage : Option < Vec < A > > , }
    };
}

Resamples!()