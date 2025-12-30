// Generated macro for IterMut (struct)
macro_rules! DepcrateIterMut {
() => {
// Module: crate
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " `ArrayDeque` mutable iterator"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct IterMut < 'a , T : 'a > { ring : & 'a mut [MaybeUninit < T >] , tail : usize , len : usize , }
};
}
