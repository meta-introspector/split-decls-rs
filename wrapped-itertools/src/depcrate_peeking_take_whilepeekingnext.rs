// Generated macro for PeekingNext (trait)
macro_rules! Depcrate_peeking_take_whilePeekingNext {
() => {
// Module: crate::peeking_take_while
// Provides: {"PeekingNext"}
// Dependencies: {}
# [doc = " An iterator that allows peeking at an element before deciding to accept it."] # [doc = ""] # [doc = " See [`.peeking_take_while()`](crate::Itertools::peeking_take_while)"] # [doc = " for more information."] # [doc = ""] # [doc = " This is implemented by peeking adaptors like peekable and put back,"] # [doc = " but also by a few iterators that can be peeked natively, like the slice’s"] # [doc = " by reference iterator ([`std::slice::Iter`])."] pub trait PeekingNext : Iterator { # [doc = " Pass a reference to the next iterator element to the closure `accept`;"] # [doc = " if `accept` returns `true`, return it as the next element,"] # [doc = " else `None`."] fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where Self : Sized , F : FnOnce (& Self :: Item) -> bool ; }
};
}
