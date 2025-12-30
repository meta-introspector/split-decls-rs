// Generated macro for impl_114 (impl)
macro_rules! Depcrate_distr_otherimpl_114 {
() => {
// Module: crate::distr::other
// Provides: {"impl_114"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl SampleString for Alphabetic { fn append_string < R : Rng + ? Sized > (& self , rng : & mut R , string : & mut String , len : usize) { unsafe { let v = string . as_mut_vec () ; v . reserve_exact (len) ; v . extend (self . sample_iter (rng) . take (len)) ; } } }
};
}
