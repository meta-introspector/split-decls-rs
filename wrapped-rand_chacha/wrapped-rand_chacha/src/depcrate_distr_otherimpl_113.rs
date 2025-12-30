// Generated macro for impl_113 (impl)
macro_rules! Depcrate_distr_otherimpl_113 {
() => {
// Module: crate::distr::other
// Provides: {"impl_113"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl SampleString for Alphanumeric { fn append_string < R : Rng + ? Sized > (& self , rng : & mut R , string : & mut String , len : usize) { unsafe { let v = string . as_mut_vec () ; v . extend (self . sample_iter (rng) . take (len) . inspect (| b | debug_assert ! (b . is_ascii_alphanumeric ())) ,) ; } } }
};
}
