// Generated macro for TryCollect (trait)
macro_rules! DepcrateTryCollect {
() => {
// Module: crate
// Provides: {"TryCollect"}
// Dependencies: {}
# [doc = " [`TryCollect`] is an extension to [`Iterator`] which allows for performing"] # [doc = " a fallible collection into a collection type."] pub trait TryCollect < A > { fn try_collect < B > (& mut self) -> Result < B , B :: Error > where B : TryFromIterator < A > ; }
};
}
