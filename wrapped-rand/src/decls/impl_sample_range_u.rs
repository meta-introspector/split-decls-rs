macro_rules! deps {
    () => {
        SampleRange!();
        Error!();
        SampleUniform!();
    };
}

macro_rules! impl_sample_range_u {
    () => {
        deps!();
        macro_rules ! impl_sample_range_u { ($ t : ty) => { impl SampleRange <$ t > for RangeTo <$ t > { # [inline] fn sample_single < R : RngCore + ? Sized > (self , rng : & mut R) -> Result <$ t , Error > { <$ t as SampleUniform >:: Sampler :: sample_single (0 , self . end , rng) } # [inline] fn is_empty (& self) -> bool { 0 == self . end } } impl SampleRange <$ t > for RangeToInclusive <$ t > { # [inline] fn sample_single < R : RngCore + ? Sized > (self , rng : & mut R) -> Result <$ t , Error > { <$ t as SampleUniform >:: Sampler :: sample_single_inclusive (0 , self . end , rng) } # [inline] fn is_empty (& self) -> bool { false } } } ; }
    };
}

impl_sample_range_u!();