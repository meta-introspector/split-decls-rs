// Generated macro for Iter (struct)
macro_rules! Depcrate_dirwalkIter {
() => {
// Module: crate::dirwalk
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator for entries in a directory walk."] # [doc = ""] # [doc = " ### Parallel Operation"] # [doc = ""] # [doc = " Note that without the `parallel` feature, the iterator becomes 'serial', which means that all entries will be traversed"] # [doc = " in advance and it cannot be interrupted unless the interrupt flag is set from another thread."] # [doc = ""] # [doc = " It's a crutch that is just there to make single-threaded applications possible at all, as it's not really an iterator"] # [doc = " anymore. If this matters, better run [Repository::dirwalk()](crate::Repository::dirwalk) by hand as it provides all"] # [doc = " control one would need, just not as an iterator."] # [doc = ""] # [doc = " Also, even with `parallel` set, the first call to `next()` will block until there is an item available, without a chance"] # [doc = " to interrupt unless the interrupt flag is set from another thread."] pub struct Iter { # [cfg (feature = "parallel")] # [allow (clippy :: type_complexity)] rx_and_join : Option < (std :: sync :: mpsc :: Receiver < iter :: Item > , std :: thread :: JoinHandle < Result < iter :: Outcome , Error > > ,) > , # [cfg (feature = "parallel")] should_interrupt : crate :: util :: OwnedOrStaticAtomicBool , # [doc = " Without parallelization, the iterator has to buffer all changes in advance."] # [cfg (not (feature = "parallel"))] items : std :: vec :: IntoIter < iter :: Item > , # [doc = " The outcome of the operation, only available once the operation has ended."] out : Option < iter :: Outcome > , }
};
}
