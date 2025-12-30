// Generated macro for impl_110 (impl)
macro_rules! Depcrate_distr_otherimpl_110 {
() => {
// Module: crate::distr::other
// Provides: {"impl_110"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl SampleString for StandardUniform { fn append_string < R : Rng + ? Sized > (& self , rng : & mut R , s : & mut String , len : usize) { s . reserve (4 * len) ; s . extend (Distribution :: < char > :: sample_iter (self , rng) . take (len)) ; } }
};
}
