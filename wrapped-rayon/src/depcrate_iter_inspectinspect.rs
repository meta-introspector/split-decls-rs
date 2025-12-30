// Generated macro for Inspect (struct)
macro_rules! Depcrate_iter_inspectInspect {
() => {
// Module: crate::iter::inspect
// Provides: {"Inspect"}
// Dependencies: {}
# [doc = " `Inspect` is an iterator that calls a function with a reference to each"] # [doc = " element before yielding it."] # [doc = ""] # [doc = " This struct is created by the [`inspect()`] method on [`ParallelIterator`]"] # [doc = ""] # [doc = " [`inspect()`]: ParallelIterator::inspect()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Clone)] pub struct Inspect < I , F > { base : I , inspect_op : F , }
};
}
