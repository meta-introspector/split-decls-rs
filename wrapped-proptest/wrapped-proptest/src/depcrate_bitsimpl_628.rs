// Generated macro for impl_628 (impl)
macro_rules! Depcrate_bitsimpl_628 {
() => {
// Module: crate::bits
// Provides: {"impl_628"}
// Dependencies: {}
impl < T : BitSetLike > Strategy for BitSetStrategy < T > { type Tree = BitSetValueTree < T > ; type Value = T ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let mut inner = T :: new_bitset (self . max) ; for bit in self . min .. self . max { if self . mask . as_ref () . map_or (true , | mask | mask . test (bit)) && runner . rng () . random () { inner . set (bit) ; } } Ok (BitSetValueTree { inner , shrink : self . min , prev_shrink : None , min_count : 0 , }) } }
};
}
