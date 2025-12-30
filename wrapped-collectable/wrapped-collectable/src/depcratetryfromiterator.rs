// Generated macro for TryFromIterator (trait)
macro_rules! DepcrateTryFromIterator {
() => {
// Module: crate
// Provides: {"TryFromIterator"}
// Dependencies: {}
# [doc = " Try to build a collection type from an [`Iterator`]."] # [doc = ""] # [doc = " Fallible in the event the capacity of the underlying container type is"] # [doc = " exceeded."] pub trait TryFromIterator < A > : Sized { # [doc = " Error type."] type Error ; # [doc = " Try to create a new collection from the given iterator, potentially"] # [doc = " returning an error if the underlying collection's capacity is exceeded."] fn try_from_iter < T > (iter : T) -> Result < Self , Self :: Error > where T : IntoIterator < Item = A > ; }
};
}
