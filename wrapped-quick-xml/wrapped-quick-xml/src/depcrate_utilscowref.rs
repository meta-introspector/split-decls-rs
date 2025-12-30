// Generated macro for CowRef (enum)
macro_rules! Depcrate_utilsCowRef {
() => {
// Module: crate::utils
// Provides: {"CowRef"}
// Dependencies: {}
# [doc = " A version of [`Cow`] that can borrow from two different buffers, one of them"] # [doc = " is a deserializer input."] # [doc = ""] # [doc = " # Lifetimes"] # [doc = ""] # [doc = " - `'i`: lifetime of the data that deserializer borrow from the parsed input"] # [doc = " - `'s`: lifetime of the data that owned by a deserializer"] pub enum CowRef < 'i , 's , B > where B : ToOwned + ? Sized , { # [doc = " An input borrowed from the parsed data"] Input (& 'i B) , # [doc = " An input borrowed from the buffer owned by another deserializer"] Slice (& 's B) , # [doc = " An input taken from an external deserializer, owned by that deserializer"] Owned (< B as ToOwned > :: Owned) , }
};
}
