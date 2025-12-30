// Generated macro for _ensure_send_and_sync (function)
macro_rules! Depcrate_ensure_send_and_sync {
() => {
// Module: crate
// Provides: {"_ensure_send_and_sync"}
// Dependencies: {}
fn _ensure_send_and_sync () { use futures_lite :: future :: pending ; fn is_send < T : Send > (_ : T) { } fn is_sync < T : Sync > (_ : T) { } fn is_static < T : 'static > (_ : T) { } is_send :: < Executor < '_ > > (Executor :: new ()) ; is_sync :: < Executor < '_ > > (Executor :: new ()) ; let ex = Executor :: new () ; let state = ex . state () ; is_send (ex . run (pending :: < () > ())) ; is_sync (ex . run (pending :: < () > ())) ; is_send (ex . tick ()) ; is_sync (ex . tick ()) ; is_send (Executor :: schedule (state)) ; is_sync (Executor :: schedule (state)) ; is_static (Executor :: schedule (state)) ; # [doc = " ```compile_fail"] # [doc = " use async_executor::LocalExecutor;"] # [doc = " use futures_lite::future::pending;"] # [doc = ""] # [doc = " fn is_send<T: Send>(_: T) {}"] # [doc = " fn is_sync<T: Sync>(_: T) {}"] # [doc = ""] # [doc = " is_send::<LocalExecutor<'_>>(LocalExecutor::new());"] # [doc = " is_sync::<LocalExecutor<'_>>(LocalExecutor::new());"] # [doc = ""] # [doc = " let ex = LocalExecutor::new();"] # [doc = " is_send(ex.run(pending::<()>()));"] # [doc = " is_sync(ex.run(pending::<()>()));"] # [doc = " is_send(ex.tick());"] # [doc = " is_sync(ex.tick());"] # [doc = " ```"] fn _negative_test () { } }
};
}
