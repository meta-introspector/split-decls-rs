// Generated macro for impl_17 (impl)
macro_rules! Depcrate_hasherimpl_17 {
() => {
// Module: crate::hasher
// Provides: {"impl_17"}
// Dependencies: {}
impl Default for AddressHasherBuilder { # [doc = " Default construct the AddressHasherBuilder."] # [doc = ""] # [doc = " The position of the slice is determined initially"] # [doc = " through random draw and then by incrementing a thread-local"] # [doc = " This way each hashmap can be expected to use a slightly different"] # [doc = " slice. This is essentially the same mechanism as what is used by"] # [doc = " `RandomState`"] fn default () -> Self { std :: thread_local ! (static OFFSET : Cell < usize > = { let mut rng = rng () ; Cell :: new (rng . random_range (0 .. ADDRESS_BYTES - mem :: size_of ::< u64 > ())) }) ; let offset = OFFSET . with (| offset | { let mut next_offset = offset . get () + 1 ; if next_offset > ADDRESS_BYTES - mem :: size_of :: < u64 > () { next_offset = 0 ; } offset . set (next_offset) ; next_offset }) ; AddressHasherBuilder { offset } } }
};
}
