// Generated macro for impl_206 (impl)
macro_rules! Depcrate_distr_uniform_otherimpl_206 {
() => {
// Module: crate::distr::uniform::other
// Provides: {"impl_206"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl SampleString for StandardUniform { fn append_string < R : Rng + ? Sized > (& self , rng : & mut R , s : & mut String , len : usize) { s . reserve (4 * len) ; s . extend (Distribution :: < char > :: sample_iter (self , rng) . take (len)) ; } }
};
}
