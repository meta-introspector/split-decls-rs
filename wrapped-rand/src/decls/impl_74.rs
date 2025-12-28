macro_rules! deps {
    () => {
        SampleString!();
        StandardUniform!();
        Rng!();
        Distribution!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl SampleString for StandardUniform { fn append_string < R : Rng + ? Sized > (& self , rng : & mut R , s : & mut String , len : usize) { s . reserve (4 * len) ; s . extend (Distribution :: < char > :: sample_iter (self , rng) . take (len)) ; } }
    };
}

impl_74!()