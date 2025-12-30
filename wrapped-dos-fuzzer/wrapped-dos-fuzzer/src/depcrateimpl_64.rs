// Generated macro for impl_64 (impl)
macro_rules! Depcrateimpl_64 {
() => {
// Module: crate
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a > Distribution < Pattern > for UniformPatterns < 'a > { fn sample < R : Rng + ? Sized > (& self , rng : & mut R) -> Pattern { fn random_seq < 'b , I : Iterator < Item = Option < & 'b Vec < u8 > > > > (iter : & mut I) -> String { String :: from_utf8 (iter . take (COMBINATIONS) . flatten () . flatten () . cloned () . collect () ,) . expect ("Bytes to String failed!") } let mut random_words = iter :: repeat_with (| | self . patterns . choose (rng)) ; Pattern { prefix : random_seq (& mut random_words) , suffix : random_seq (& mut random_words) , repeating_pattern : random_seq (& mut random_words) , } } }
};
}
