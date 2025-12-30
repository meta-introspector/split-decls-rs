// Generated macro for TakeWhileRef (struct)
macro_rules! Depcrate_adaptorsTakeWhileRef {
() => {
// Module: crate::adaptors
// Provides: {"TakeWhileRef"}
// Dependencies: {}
# [doc = " An iterator adaptor that borrows from a `Clone`-able iterator"] # [doc = " to only pick off elements while the predicate returns `true`."] # [doc = ""] # [doc = " See [`.take_while_ref()`](crate::Itertools::take_while_ref) for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct TakeWhileRef < 'a , I : 'a , F > { iter : & 'a mut I , f : F , }
};
}
