macro_rules! deps {
    () => {
        SampleUniform!();
        Error!();
        SampleRange!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < T : SampleUniform + PartialOrd > SampleRange < T > for Range < T > { # [inline] fn sample_single < R : RngCore + ? Sized > (self , rng : & mut R) -> Result < T , Error > { T :: Sampler :: sample_single (self . start , self . end , rng) } # [inline] fn is_empty (& self) -> bool { ! (self . start < self . end) } }
    };
}

impl_173!()