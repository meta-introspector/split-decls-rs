// Generated macro for join_all (function)
macro_rules! Depcrate_future_extjoin_all {
() => {
// Module: crate::future_ext
// Provides: {"join_all"}
// Dependencies: {}
# [doc = " Wraps futures::future::join_all to ensure that the futures are only polled if they are woken."] pub fn join_all < I > (iter : I) -> futures :: future :: JoinAll < Wakened < I :: Item > > where I : IntoIterator , I :: Item : Future , { futures :: future :: join_all (iter . into_iter () . map (| f | f . wakened ())) }
};
}
