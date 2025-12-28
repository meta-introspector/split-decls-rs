macro_rules! deps {
    () => {
        Wnaf!();
        Group!();
    };
}

macro_rules! WnafGroup {
    () => {
        deps!();
        # [doc = " Extension trait on a [`Group`] that provides helpers used by [`Wnaf`]."] pub trait WnafGroup : Group { # [doc = " Recommends a wNAF window size given the number of scalars you intend to multiply"] # [doc = " a base by. Always returns a number between 2 and 22, inclusive."] fn recommended_wnaf_for_num_scalars (num_scalars : usize) -> usize ; }
    };
}

WnafGroup!()