// Generated macro for join4 (function)
macro_rules! Depcrate_future_extjoin4 {
() => {
// Module: crate::future_ext
// Provides: {"join4"}
// Dependencies: {}
# [doc = " Wraps futures::future::join4 to ensure that the futures are only polled if they are woken."] pub fn join4 < Fut1 , Fut2 , Fut3 , Fut4 > (future1 : Fut1 , future2 : Fut2 , future3 : Fut3 , future4 : Fut4 ,) -> futures :: future :: Join4 < Wakened < Fut1 > , Wakened < Fut2 > , Wakened < Fut3 > , Wakened < Fut4 > > where Fut1 : Future , Fut2 : Future , Fut3 : Future , Fut4 : Future , { futures :: future :: join4 (future1 . wakened () , future2 . wakened () , future3 . wakened () , future4 . wakened () ,) }
};
}
