// Generated macro for impl_104 (impl)
macro_rules! Depcrate_random_stateimpl_104 {
() => {
// Module: crate::random_state
// Provides: {"impl_104"}
// Dependencies: {}
impl RandomState { # [doc = " Create a new `RandomState` `BuildHasher` using random keys."] # [doc = ""] # [doc = " Each instance will have a unique set of keys derived from [RandomSource]."] # [doc = ""] # [inline] pub fn new () -> RandomState { let src = get_src () ; let fixed = get_fixed_seeds () ; Self :: from_keys (& fixed [0] , & fixed [1] , src . gen_hasher_seed ()) } # [doc = " Create a new `RandomState` `BuildHasher` based on the provided seeds, but in such a way"] # [doc = " that each time it is called the resulting state will be different and of high quality."] # [doc = " This allows fixed constant or poor quality seeds to be provided without the problem of different"] # [doc = " `BuildHasher`s being identical or weak."] # [doc = ""] # [doc = " This is done via permuting the provided values with the value of a static counter and memory address."] # [doc = " (This makes this method somewhat more expensive than `with_seeds` below which does not do this)."] # [doc = ""] # [doc = " The provided values (k0-k3) do not need to be of high quality but they should not all be the same value."] # [inline] pub fn generate_with (k0 : u64 , k1 : u64 , k2 : u64 , k3 : u64) -> RandomState { let src = get_src () ; let fixed = get_fixed_seeds () ; RandomState :: from_keys (& fixed [0] , & [k0 , k1 , k2 , k3] , src . gen_hasher_seed ()) } fn from_keys (a : & [u64 ; 4] , b : & [u64 ; 4] , c : usize) -> RandomState { let & [k0 , k1 , k2 , k3] = a ; let mut hasher = AHasher :: from_random_state (& RandomState { k0 , k1 , k2 , k3 }) ; hasher . write_usize (c) ; let mix = | l : u64 , r : u64 | { let mut h = hasher . clone () ; h . write_u64 (l) ; h . write_u64 (r) ; h . finish () } ; RandomState { k0 : mix (b [0] , b [2]) , k1 : mix (b [1] , b [3]) , k2 : mix (b [2] , b [1]) , k3 : mix (b [3] , b [0]) , } } # [doc = " Internal. Used by Default."] # [inline] pub (crate) fn with_fixed_keys () -> RandomState { let [k0 , k1 , k2 , k3] = get_fixed_seeds () [0] ; RandomState { k0 , k1 , k2 , k3 } } # [doc = " Build a `RandomState` from a single key. The provided key does not need to be of high quality,"] # [doc = " but all `RandomState`s created from the same key will produce identical hashers."] # [doc = " (In contrast to `generate_with` above)"] # [doc = ""] # [doc = " This allows for explicitly setting the seed to be used."] # [doc = ""] # [doc = " Note: This method does not require the provided seed to be strong."] # [inline] pub fn with_seed (key : usize) -> RandomState { let fixed = get_fixed_seeds () ; RandomState :: from_keys (& fixed [0] , & fixed [1] , key) } # [doc = " Allows for explicitly setting the seeds to be used."] # [doc = " All `RandomState`s created with the same set of keys will produce identical hashers."] # [doc = " (In contrast to `generate_with` above)"] # [doc = ""] # [doc = " Note: If DOS resistance is desired one of these should be a decent quality random number."] # [doc = " If 4 high quality random numbers are not cheaply available this method is robust against 0s being passed for"] # [doc = " one or more of the parameters or the same value being passed for more than one parameter."] # [doc = " It is recommended to pass numbers in order from highest to lowest quality (if there is any difference)."] # [inline] pub const fn with_seeds (k0 : u64 , k1 : u64 , k2 : u64 , k3 : u64) -> RandomState { RandomState { k0 : k0 ^ PI2 [0] , k1 : k1 ^ PI2 [1] , k2 : k2 ^ PI2 [2] , k3 : k3 ^ PI2 [3] , } } # [doc = " Calculates the hash of a single value. This provides a more convenient (and faster) way to obtain a hash:"] # [doc = " For example:"] # [cfg_attr (feature = "std" , doc = r##" # Examples
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
    "##)] # [doc = " (Note that these two ways to get a hash may not produce the same value for the same data)"] # [doc = ""] # [doc = " This is intended as a convenience for code which *consumes* hashes, such"] # [doc = " as the implementation of a hash table or in unit tests that check"] # [doc = " whether a custom [`Hash`] implementation behaves as expected."] # [doc = ""] # [doc = " This must not be used in any code which *creates* hashes, such as in an"] # [doc = " implementation of [`Hash`].  The way to create a combined hash of"] # [doc = " multiple values is to call [`Hash::hash`] multiple times using the same"] # [doc = " [`Hasher`], not to call this method repeatedly and combine the results."] # [inline] pub fn hash_one < T : Hash > (& self , x : T) -> u64 where Self : Sized , { use crate :: specialize :: CallHasher ; T :: get_hash (& x , self) } }
};
}
