// Generated macro for join3 (function)
macro_rules! Depcrate_future_extjoin3 {
() => {
// Module: crate::future_ext
// Provides: {"join3"}
// Dependencies: {}
# [doc = " Wraps futures::future::join3 to ensure that the futures are only polled if they are woken."] pub fn join3 < Fut1 , Fut2 , Fut3 > (future1 : Fut1 , future2 : Fut2 , future3 : Fut3 ,) -> futures :: future :: Join3 < Wakened < Fut1 > , Wakened < Fut2 > , Wakened < Fut3 > > where Fut1 : Future , Fut2 : Future , Fut3 : Future , { futures :: future :: join3 (future1 . wakened () , future2 . wakened () , future3 . wakened ()) }
};
}
