// Generated macro for join (function)
macro_rules! Depcrate_future_extjoin {
() => {
// Module: crate::future_ext
// Provides: {"join"}
// Dependencies: {}
# [doc = " Wraps futures::future::join to ensure that the futures are only polled if they are woken."] pub fn join < Fut1 , Fut2 > (future1 : Fut1 , future2 : Fut2 ,) -> futures :: future :: Join < Wakened < Fut1 > , Wakened < Fut2 > > where Fut1 : Future , Fut2 : Future , { futures :: future :: join (future1 . wakened () , future2 . wakened ()) }
};
}
