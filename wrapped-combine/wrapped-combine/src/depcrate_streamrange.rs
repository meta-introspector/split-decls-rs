// Generated macro for Range (trait)
macro_rules! Depcrate_streamRange {
() => {
// Module: crate::stream
// Provides: {"Range"}
// Dependencies: {}
# [doc = " Trait representing a range of elements."] pub trait Range { # [doc = " Returns the remaining length of `self`."] # [doc = " The returned length need not be the same as the number of items left in the stream."] fn len (& self) -> usize ; # [doc = " Returns `true` if the range does not contain any elements (`Range::len() == 0`)"] fn is_empty (& self) -> bool { self . len () == 0 } }
};
}
