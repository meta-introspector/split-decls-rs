// Generated macro for print_alloc_stats (function)
macro_rules! Depcrate_executor_alloc_statsprint_alloc_stats {
() => {
// Module: crate::executor::alloc_stats
// Provides: {"print_alloc_stats"}
// Dependencies: {}
async fn print_alloc_stats () { future :: poll_fn (| cx | { let talc = ALLOCATOR . lock () ; debug ! ("<alloc-stats>\n{}" , talc . get_counters ()) ; cx . waker () . wake_by_ref () ; Poll :: < () > :: Pending }) . await ; }
};
}
