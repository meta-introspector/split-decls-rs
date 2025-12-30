// Generated macro for impl_106 (impl)
macro_rules! Depcrate_random_stateimpl_106 {
() => {
// Module: crate::random_state
// Provides: {"impl_106"}
// Dependencies: {}
impl BuildHasher for RandomState { type Hasher = AHasher ; # [doc = " Constructs a new [AHasher] with keys based on this [RandomState] object."] # [doc = " This means that two different [RandomState]s will generate"] # [doc = " [AHasher]s that will return different hashcodes, but [Hasher]s created from the same [BuildHasher]"] # [doc = " will generate the same hashes for the same input data."] # [doc = ""] # [cfg_attr (feature = "std" , doc = r##" # Examples
```
        use ahash::{AHasher, RandomState};
        use std::hash::{Hasher, BuildHasher};

        let build_hasher = RandomState::new();
        let mut hasher_1 = build_hasher.build_hasher();
        let mut hasher_2 = build_hasher.build_hasher();

        hasher_1.write_u32(1234);
        hasher_2.write_u32(1234);

        assert_eq!(hasher_1.finish(), hasher_2.finish());

        let other_build_hasher = RandomState::new();
        let mut different_hasher = other_build_hasher.build_hasher();
        different_hasher.write_u32(1234);
        assert_ne!(different_hasher.finish(), hasher_1.finish());
```
    "##)] # [doc = " [Hasher]: std::hash::Hasher"] # [doc = " [BuildHasher]: std::hash::BuildHasher"] # [doc = " [HashMap]: std::collections::HashMap"] # [inline] fn build_hasher (& self) -> AHasher { AHasher :: from_random_state (self) } # [doc = " Calculates the hash of a single value. This provides a more convenient (and faster) way to obtain a hash:"] # [doc = " For example:"] # [cfg_attr (feature = "std" , doc = r##" # Examples
```
    use std::hash::BuildHasher;
    use ahash::RandomState;

    let hash_builder = RandomState::new();
    let hash = hash_builder.hash_one("Some Data");
```
    "##)] # [doc = " This is similar to:"] # [cfg_attr (feature = "std" , doc = r##" # Examples
```
    use std::hash::{BuildHasher, Hash, Hasher};
    use ahash::RandomState;

    let hash_builder = RandomState::new();
    let mut hasher = hash_builder.build_hasher();
    "Some Data".hash(&mut hasher);
    let hash = hasher.finish();
```
    "##)] # [doc = " (Note that these two ways to get a hash may not produce the same value for the same data)"] # [doc = ""] # [doc = " This is intended as a convenience for code which *consumes* hashes, such"] # [doc = " as the implementation of a hash table or in unit tests that check"] # [doc = " whether a custom [`Hash`] implementation behaves as expected."] # [doc = ""] # [doc = " This must not be used in any code which *creates* hashes, such as in an"] # [doc = " implementation of [`Hash`].  The way to create a combined hash of"] # [doc = " multiple values is to call [`Hash::hash`] multiple times using the same"] # [doc = " [`Hasher`], not to call this method repeatedly and combine the results."] # [cfg (specialize)] # [inline] fn hash_one < T : Hash > (& self , x : T) -> u64 { RandomState :: hash_one (self , x) } }
};
}
