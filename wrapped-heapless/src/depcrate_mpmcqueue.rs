// Generated macro for Queue (type)
macro_rules! Depcrate_mpmcQueue {
() => {
// Module: crate::mpmc
// Provides: {"Queue"}
// Dependencies: {}
# [doc = " A statically allocated multi-producer, multi-consumer queue with a capacity of `N` elements."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " `N` must be a power of 2."] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " The maximum value of `N` is 128 if the `mpmc_large` feature is not enabled."] pub type Queue < T , const N : usize > = QueueInner < T , OwnedStorage < N > > ;
};
}
