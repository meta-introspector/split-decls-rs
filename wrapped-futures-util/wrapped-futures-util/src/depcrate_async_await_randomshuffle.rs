// Generated macro for shuffle (function)
macro_rules! Depcrate_async_await_randomshuffle {
() => {
// Module: crate::async_await::random
// Provides: {"shuffle"}
// Dependencies: {}
# [doc (hidden)] pub fn shuffle < T > (slice : & mut [T]) { for i in (1 .. slice . len ()) . rev () { slice . swap (i , gen_index (i + 1)) ; } }
};
}
