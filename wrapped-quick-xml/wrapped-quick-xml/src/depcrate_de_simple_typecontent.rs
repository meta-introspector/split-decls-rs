// Generated macro for Content (enum)
macro_rules! Depcrate_de_simple_typeContent {
() => {
// Module: crate::de::simple_type
// Provides: {"Content"}
// Dependencies: {}
# [doc = " A version of [`Cow`] that can borrow from two different buffers, one of them"] # [doc = " is a deserializer input, and conceptually contains only part of owned data."] # [doc = ""] # [doc = " # Lifetimes"] # [doc = " - `'de` -- lifetime of the data that deserializer borrow from the parsed input"] # [doc = " - `'a` -- lifetime of the data that owned by a deserializer"] enum Content < 'de , 'a > { # [doc = " An input borrowed from the parsed data"] Input (& 'de str) , # [doc = " An input borrowed from the buffer owned by another deserializer"] Slice (& 'a str) , # [doc = " An input taken from an external deserializer, owned by that deserializer."] # [doc = " Only part of this data, located after offset represented by `usize`, used"] # [doc = " to deserialize data, the other is a garbage that can't be dropped because"] # [doc = " we do not want to make reallocations if they will not required."] Owned (String , usize) , }
};
}
