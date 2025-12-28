macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! other_48 {
    () => {
        deps!();
        macro path_std ($ ($ x : tt) *) { generic :: ty :: Path :: new (pathvec_std ! ($ ($ x) *)) }
    };
}

other_48!()