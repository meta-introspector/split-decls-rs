// Generated macro for Iter (struct)
macro_rules! Depcrate_arrayvecIter {
() => {
// Module: crate::arrayvec
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Iterator over the elements of an [`ArrayVec`]."] # [derive (Clone , Debug)] pub struct Iter < 'a , T > { # [doc = " Decoder which iterates over the elements of the message."] elements : & 'a [Option < T >] , # [doc = " Position within the iterator."] position : usize , }
};
}
