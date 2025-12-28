macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! SampleRange {
    () => {
        deps!();
        # [doc = " Range that supports generating a single sample efficiently."] # [doc = ""] # [doc = " Any type implementing this trait can be used to specify the sampled range"] # [doc = " for `Rng::random_range`."] pub trait SampleRange < T > { # [doc = " Generate a sample from the given range."] fn sample_single < R : RngCore + ? Sized > (self , rng : & mut R) -> Result < T , Error > ; # [doc = " Check whether the range is empty."] fn is_empty (& self) -> bool ; }
    };
}

SampleRange!()