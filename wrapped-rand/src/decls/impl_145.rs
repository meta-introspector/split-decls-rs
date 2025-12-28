macro_rules! deps {
    () => {
        StandardUniform!();
        Distribution!();
        Rng!();
        SampleString!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl SampleString for StandardUniform { fn append_string < R : Rng + ? Sized > (& self , rng : & mut R , s : & mut String , len : usize) { s . reserve (4 * len) ; s . extend (Distribution :: < char > :: sample_iter (self , rng) . take (len)) ; } }
    };
}

impl_145!();