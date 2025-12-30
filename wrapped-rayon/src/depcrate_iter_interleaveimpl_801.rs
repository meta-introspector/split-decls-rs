// Generated macro for impl_801 (impl)
macro_rules! Depcrate_iter_interleaveimpl_801 {
() => {
// Module: crate::iter::interleave
// Provides: {"impl_801"}
// Dependencies: {}
# [doc = " Iterator implementation for InterleaveSeq. This implementation is"] # [doc = " taken more or less verbatim from itertools. It is replicated here"] # [doc = " (instead of calling itertools directly), because we also need to"] # [doc = " implement `DoubledEndedIterator` and `ExactSizeIterator`."] impl < I , J > Iterator for InterleaveSeq < I , J > where I : Iterator , J : Iterator < Item = I :: Item > , { type Item = I :: Item ; # [inline] fn next (& mut self) -> Option < Self :: Item > { self . i_next = ! self . i_next ; if self . i_next { match self . i . next () { None => self . j . next () , r => r , } } else { match self . j . next () { None => self . i . next () , r => r , } } } fn size_hint (& self) -> (usize , Option < usize >) { let (ih , jh) = (self . i . size_hint () , self . j . size_hint ()) ; let min = ih . 0 . saturating_add (jh . 0) ; let max = match (ih . 1 , jh . 1) { (Some (x) , Some (y)) => x . checked_add (y) , _ => None , } ; (min , max) } }
};
}
