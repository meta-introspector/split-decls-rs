// Generated macro for wake_all (function)
macro_rules! Depcrate_connectionwake_all {
() => {
// Module: crate::connection
// Provides: {"wake_all"}
// Dependencies: {}
fn wake_all (wakers : & mut FxHashMap < StreamId , Waker >) { wakers . drain () . for_each (| (_ , waker) | waker . wake ()) }
};
}
