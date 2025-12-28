macro_rules! deps {
    () => {
        SampleString!();
        Rng!();
        Alphanumeric!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl SampleString for Alphanumeric { fn append_string < R : Rng + ? Sized > (& self , rng : & mut R , string : & mut String , len : usize) { unsafe { let v = string . as_mut_vec () ; v . extend (self . sample_iter (rng) . take (len) . inspect (| b | debug_assert ! (b . is_ascii_alphanumeric ())) ,) ; } } }
    };
}

impl_77!();