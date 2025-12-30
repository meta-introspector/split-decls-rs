// Generated macro for impl_25 (impl)
macro_rules! Depcrate_arbitraryimpl_25 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_25"}
// Dependencies: {}
impl Gen { # [doc = " Returns a `Gen` with the given size configuration."] # [doc = ""] # [doc = " The `size` parameter controls the size of random values generated."] # [doc = " For example, it specifies the maximum length of a randomly generated"] # [doc = " vector, but is and should not be used to control the range of a"] # [doc = " randomly generated number. (Unless that number is used to control the"] # [doc = " size of a data structure.)"] pub fn new (size : usize) -> Gen { Gen { rng : rand :: rngs :: SmallRng :: from_os_rng () , size } } # [doc = " Returns the size configured with this generator."] pub fn size (& self) -> usize { self . size } # [doc = " Choose among the possible alternatives in the slice given. If the slice"] # [doc = " is empty, then `None` is returned. Otherwise, a non-`None` value is"] # [doc = " guaranteed to be returned."] pub fn choose < 'a , T > (& mut self , slice : & 'a [T]) -> Option < & 'a T > { slice . choose (& mut self . rng) } fn random < T > (& mut self) -> T where rand :: distr :: StandardUniform : rand :: distr :: Distribution < T > , { self . rng . random () } fn random_range < T , R > (& mut self , range : R) -> T where T : rand :: distr :: uniform :: SampleUniform , R : rand :: distr :: uniform :: SampleRange < T > , { self . rng . random_range (range) } }
};
}
