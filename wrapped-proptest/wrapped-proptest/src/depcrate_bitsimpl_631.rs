// Generated macro for impl_631 (impl)
macro_rules! Depcrate_bitsimpl_631 {
() => {
// Module: crate::bits
// Provides: {"impl_631"}
// Dependencies: {}
impl < T : BitSetLike > Strategy for SampledBitSetStrategy < T > { type Tree = BitSetValueTree < T > ; type Value = T ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { let mut bits = T :: new_bitset (self . bits . end_excl ()) ; let count = sample_uniform_incl (runner , self . size . start () , self . size . end_incl () ,) ; if bits . len () < count { panic ! ("not enough bits to sample") ; } for bit in self . bits . iter () . choose_multiple (runner . rng () , count) { bits . set (bit) ; } Ok (BitSetValueTree { inner : bits , shrink : self . bits . start () , prev_shrink : None , min_count : self . size . start () , }) } }
};
}
