// Generated macro for run_executor (function)
macro_rules! Depcrate_local_poolrun_executor {
() => {
// Module: crate::local_pool
// Provides: {"run_executor"}
// Dependencies: {}
fn run_executor < T , F : FnMut (& mut Context < '_ >) -> Poll < T > > (mut f : F) -> T { let _enter = enter () . expect ("cannot execute `LocalPool` executor from within \
         another executor" ,) ; CURRENT_THREAD_NOTIFY . with (| thread_notify | { let waker = waker_ref (thread_notify) ; let mut cx = Context :: from_waker (& waker) ; loop { if let Poll :: Ready (t) = f (& mut cx) { return t ; } while ! thread_notify . unparked . swap (false , Ordering :: Acquire) { thread :: park () ; } } }) }
};
}
