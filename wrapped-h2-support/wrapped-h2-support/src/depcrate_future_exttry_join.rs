// Generated macro for try_join (function)
macro_rules! Depcrate_future_exttry_join {
() => {
// Module: crate::future_ext
// Provides: {"try_join"}
// Dependencies: {}
# [doc = " Wraps futures::future::try_join to ensure that the futures are only polled if they are woken."] pub fn try_join < Fut1 , Fut2 > (future1 : Fut1 , future2 : Fut2 ,) -> futures :: future :: TryJoin < Wakened < Fut1 > , Wakened < Fut2 > > where Fut1 : futures :: future :: TryFuture + Future , Fut2 : Future , Wakened < Fut1 > : futures :: future :: TryFuture , Wakened < Fut2 > : futures :: future :: TryFuture < Error = < Wakened < Fut1 > as TryFuture > :: Error > , { futures :: future :: try_join (future1 . wakened () , future2 . wakened ()) }
};
}
