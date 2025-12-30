// Generated macro for Iter (struct)
macro_rules! DepcrateIter {
() => {
// Module: crate
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " `ArrayDeque` iterator"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct Iter < 'a , T : 'a > { ring : & 'a [MaybeUninit < T >] , tail : usize , len : usize , }
};
}
