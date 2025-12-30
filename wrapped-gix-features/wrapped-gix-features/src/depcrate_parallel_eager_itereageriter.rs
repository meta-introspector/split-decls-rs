// Generated macro for EagerIter (struct)
macro_rules! Depcrate_parallel_eager_iterEagerIter {
() => {
// Module: crate::parallel::eager_iter
// Provides: {"EagerIter"}
// Dependencies: {}
# [doc = " Evaluate any iterator in their own thread."] # [doc = ""] # [doc = " This is particularly useful if the wrapped iterator performs IO and/or heavy computations."] # [doc = " Use [`EagerIter::new()`] for instantiation."] pub struct EagerIter < I : Iterator > { receiver : std :: sync :: mpsc :: Receiver < Vec < I :: Item > > , chunk : Option < std :: vec :: IntoIter < I :: Item > > , size_hint : (usize , Option < usize >) , }
};
}
