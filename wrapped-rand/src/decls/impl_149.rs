macro_rules! deps {
    () => {
        SampleString!();
        Rng!();
        Alphabetic!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl SampleString for Alphabetic { fn append_string < R : Rng + ? Sized > (& self , rng : & mut R , string : & mut String , len : usize) { unsafe { let v = string . as_mut_vec () ; v . reserve_exact (len) ; v . extend (self . sample_iter (rng) . take (len)) ; } } }
    };
}

impl_149!()