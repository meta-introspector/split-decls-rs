// Generated macro for IterMut (struct)
macro_rules! DepcrateIterMut {
() => {
// Module: crate
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " An iterator for mutable references to the bits in a `BitVec`."] pub struct IterMut < 'a , B : 'a + BitBlock = u32 > { vec : Rc < RefCell < & 'a mut BitVec < B > > > , range : Range < usize > , }
};
}
