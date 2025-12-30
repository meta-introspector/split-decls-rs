// Generated macro for impl_386 (impl)
macro_rules! Depcrate_reference_iterimpl_386 {
() => {
// Module: crate::reference::iter
// Provides: {"impl_386"}
// Dependencies: {}
impl < 'packed , 'repo > Iter < 'packed , 'repo > { fn new (repo : & 'repo crate :: Repository , platform : gix_ref :: file :: iter :: LooseThenPacked < 'packed , 'repo >) -> Self { Iter { inner : platform , peel_with_packed : None , peel : false , repo , } } }
};
}
