// Generated macro for Selector (struct)
macro_rules! Depcrate_sampleSelector {
() => {
// Module: crate::sample
// Provides: {"Selector"}
// Dependencies: {}
# [doc = " A value for picking random values out of iterators."] # [doc = ""] # [doc = " This is, in a sense, a more flexible variant of"] # [doc = " [`Index`](struct.Index.html) in that it can operate on arbitrary"] # [doc = " `IntoIterator` values."] # [doc = ""] # [doc = " Initially, the selection is roughly uniform, with a very slight bias"] # [doc = " towards items earlier in the iterator."] # [doc = ""] # [doc = " Shrinking causes the selection to move toward items earlier in the"] # [doc = " iterator, ultimately settling on the very first, but this currently happens"] # [doc = " in a very haphazard way that may fail to find the earliest failing input."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " Generate a non-indexable collection and a value to pick out of it."] # [doc = ""] # [doc = " ```"] # [doc = " use proptest::prelude::*;"] # [doc = ""] # [doc = " proptest! {"] # [doc = "     # /*"] # [doc = "     #[test]"] # [doc = "     # */"] # [doc = "     fn my_test("] # [doc = "         names in prop::collection::hash_set(\"[a-z]+\", 10..20),"] # [doc = "         selector in any::<prop::sample::Selector>()"] # [doc = "     ) {"] # [doc = "         println!(\"Selected name: {}\", selector.select(&names));"] # [doc = "         // Test stuff..."] # [doc = "     }"] # [doc = " }"] # [doc = " #"] # [doc = " # fn main() { my_test(); }"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Selector { rng : TestRng , bias_increment : u64 , }
};
}
