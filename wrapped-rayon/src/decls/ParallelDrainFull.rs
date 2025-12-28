macro_rules! deps {
    () => {
        Iter!();
        ParallelIterator!();
        ParallelDrainRange!();
    };
}

macro_rules! ParallelDrainFull {
    () => {
        deps!();
        # [doc = " `ParallelDrainFull` creates a parallel iterator that moves all items"] # [doc = " from a collection while retaining the original capacity."] # [doc = ""] # [doc = " Types which are indexable typically implement [`ParallelDrainRange`]"] # [doc = " instead, where you can drain fully with `par_drain(..)`."] pub trait ParallelDrainFull { # [doc = " The draining parallel iterator type that will be created."] type Iter : ParallelIterator < Item = Self :: Item > ; # [doc = " The type of item that the parallel iterator will produce."] # [doc = " This is usually the same as `IntoParallelIterator::Item`."] type Item : Send ; # [doc = " Returns a draining parallel iterator over an entire collection."] # [doc = ""] # [doc = " When the iterator is dropped, all items are removed, even if the"] # [doc = " iterator was not fully consumed. If the iterator is leaked, for example"] # [doc = " using `std::mem::forget`, it is unspecified how many items are removed."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rayon::prelude::*;"] # [doc = " use std::collections::{BinaryHeap, HashSet};"] # [doc = ""] # [doc = " let squares: HashSet<i32> = (0..10).map(|x| x * x).collect();"] # [doc = ""] # [doc = " let mut heap: BinaryHeap<_> = squares.iter().copied().collect();"] # [doc = " assert_eq!("] # [doc = "     // heaps are drained in arbitrary order"] # [doc = "     heap.par_drain()"] # [doc = "         .inspect(|x| assert!(squares.contains(x)))"] # [doc = "         .count(),"] # [doc = "     squares.len(),"] # [doc = " );"] # [doc = " assert!(heap.is_empty());"] # [doc = " assert!(heap.capacity() >= squares.len());"] # [doc = " ```"] fn par_drain (self) -> Self :: Iter ; }
    };
}

ParallelDrainFull!()