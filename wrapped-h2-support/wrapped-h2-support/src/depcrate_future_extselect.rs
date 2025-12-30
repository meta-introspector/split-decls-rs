// Generated macro for select (function)
macro_rules! Depcrate_future_extselect {
() => {
// Module: crate::future_ext
// Provides: {"select"}
// Dependencies: {}
# [doc = " Wraps futures::future::select to ensure that the futures are only polled if they are woken."] pub fn select < A , B > (future1 : A , future2 : B) -> futures :: future :: Select < Wakened < A > , Wakened < B > > where A : Future + Unpin , B : Future + Unpin , { futures :: future :: select (future1 . wakened () , future2 . wakened ()) }
};
}
