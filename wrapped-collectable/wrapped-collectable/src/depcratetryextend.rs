// Generated macro for TryExtend (trait)
macro_rules! DepcrateTryExtend {
() => {
// Module: crate
// Provides: {"TryExtend"}
// Dependencies: {}
# [doc = " Fallible equivalent of [`core::iter::Extend`] - extends a collection"] # [doc = " with the contents of an iterator, but with the option to return an error"] # [doc = " in the event the container's capacity has been exceeded."] # [doc = ""] # [doc = " [`core::iter::Extend`]: https://doc.rust-lang.org/core/iter/trait.Extend.html"] pub trait TryExtend < A > { # [doc = " Error type."] type Error ; # [doc = " Try to extend the collection from the given iterator."] fn try_extend < T > (& mut self , iter : T) -> Result < () , Self :: Error > where T : IntoIterator < Item = A > ; # [doc = " Try to extend the collection from the given slice."] fn try_extend_from_slice (& mut self , slice : & [A]) -> Result < () , Self :: Error > where A : Clone , { self . try_extend (slice . iter () . cloned ()) } }
};
}
