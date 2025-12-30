// Generated macro for PeekingTakeWhile (struct)
macro_rules! Depcrate_peeking_take_whilePeekingTakeWhile {
() => {
// Module: crate::peeking_take_while
// Provides: {"PeekingTakeWhile"}
// Dependencies: {}
# [doc = " An iterator adaptor that takes items while a closure returns `true`."] # [doc = ""] # [doc = " See [`.peeking_take_while()`](crate::Itertools::peeking_take_while)"] # [doc = " for more information."] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct PeekingTakeWhile < 'a , I , F > where I : Iterator + 'a , { iter : & 'a mut I , f : F , }
};
}
