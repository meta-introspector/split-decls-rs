// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl EpochRewardsHasher { # [doc = " Use SipHasher13 keyed on the `seed` for calculating epoch reward partition"] pub fn new (partitions : usize , seed : & Hash) -> Self { let mut hasher = SipHasher13 :: new () ; hasher . write (seed . as_ref ()) ; Self { hasher , partitions } } # [doc = " Return partition index (0..partitions) by hashing `address` with the `hasher`"] pub fn hash_address_to_partition (self , address : & Address) -> usize { let Self { mut hasher , partitions , } = self ; hasher . write (address . as_ref ()) ; let hash64 = hasher . finish () ; hash_to_partition (hash64 , partitions) } }
};
}
