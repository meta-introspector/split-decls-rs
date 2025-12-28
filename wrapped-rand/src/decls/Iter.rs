macro_rules! deps {
    () => {
        Distribution!();
    };
}

macro_rules! Iter {
    () => {
        deps!();
        # [doc = " An iterator over a [`Distribution`]"] # [doc = ""] # [doc = " This iterator yields random values of type `T` with distribution `D`"] # [doc = " from a random generator of type `R`."] # [doc = ""] # [doc = " Construct this `struct` using [`Distribution::sample_iter`] or"] # [doc = " [`Rng::sample_iter`]. It is also used by [`Rng::random_iter`] and"] # [doc = " [`crate::random_iter`]."] # [derive (Debug)] pub struct Iter < D , R , T > { distr : D , rng : R , phantom : core :: marker :: PhantomData < T > , }
    };
}

Iter!();