// Generated macro for Iter (struct)
macro_rules! Depcrate_reference_iterIter {
() => {
// Module: crate::reference::iter
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over references, with or without filter."] pub struct Iter < 'packed , 'repo > { inner : gix_ref :: file :: iter :: LooseThenPacked < 'packed , 'repo > , peel_with_packed : Option < gix_ref :: file :: packed :: SharedBufferSnapshot > , peel : bool , repo : & 'repo crate :: Repository , }
};
}
