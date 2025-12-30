// Generated macro for impl_209 (impl)
macro_rules! Depcrate_distr_uniform_otherimpl_209 {
() => {
// Module: crate::distr::uniform::other
// Provides: {"impl_209"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl SampleString for Alphanumeric { fn append_string < R : Rng + ? Sized > (& self , rng : & mut R , string : & mut String , len : usize) { unsafe { let v = string . as_mut_vec () ; v . extend (self . sample_iter (rng) . take (len) . inspect (| b | debug_assert ! (b . is_ascii_alphanumeric ())) ,) ; } } }
};
}
