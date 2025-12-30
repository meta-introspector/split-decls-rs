// Generated macro for Intersperse (struct)
macro_rules! Depcrate_iter_intersperseIntersperse {
() => {
// Module: crate::iter::intersperse
// Provides: {"Intersperse"}
// Dependencies: {}
# [doc = " `Intersperse` is an iterator that inserts a particular item between each"] # [doc = " item of the adapted iterator.  This struct is created by the"] # [doc = " [`intersperse()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`intersperse()`]: ParallelIterator::intersperse()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone , Debug)] pub struct Intersperse < I > where I : ParallelIterator , { base : I , item : I :: Item , }
};
}
