macro_rules! deps {
    () => {
        Path!();
    };
}

macro_rules! other_46 {
    () => {
        deps!();
        macro path_local ($ x : ident) { generic :: ty :: Path :: new_local (sym ::$ x) }
    };
}

other_46!();