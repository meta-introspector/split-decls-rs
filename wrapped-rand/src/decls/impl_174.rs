macro_rules! deps {
    () => {
        SampleRange!();
        SampleUniform!();
        Error!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < T : SampleUniform + PartialOrd > SampleRange < T > for RangeInclusive < T > { # [inline] fn sample_single < R : RngCore + ? Sized > (self , rng : & mut R) -> Result < T , Error > { T :: Sampler :: sample_single_inclusive (self . start () , self . end () , rng) } # [inline] fn is_empty (& self) -> bool { ! (self . start () <= self . end ()) } }
    };
}

impl_174!()