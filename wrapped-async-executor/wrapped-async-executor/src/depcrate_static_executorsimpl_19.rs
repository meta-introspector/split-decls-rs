// Generated macro for impl_19 (impl)
macro_rules! Depcrate_static_executorsimpl_19 {
() => {
// Module: crate::static_executors
// Provides: {"impl_19"}
// Dependencies: {}
impl Executor < 'static > { # [doc = " Consumes the [`Executor`] and intentionally leaks it."] # [doc = ""] # [doc = " Largely equivalent to calling `Box::leak(Box::new(executor))`, but the produced"] # [doc = " [`StaticExecutor`]'s functions are optimized to require fewer synchronizing operations"] # [doc = " when spawning, running, and finishing tasks."] # [doc = ""] # [doc = " `StaticExecutor` cannot be converted back into a `Executor`, so this operation is"] # [doc = " irreversible without the use of unsafe."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use async_executor::Executor;"] # [doc = " use futures_lite::future;"] # [doc = ""] # [doc = " let ex = Executor::new().leak();"] # [doc = ""] # [doc = " let task = ex.spawn(async {"] # [doc = "     println!(\"Hello world\");"] # [doc = " });"] # [doc = ""] # [doc = " future::block_on(ex.run(task));"] # [doc = " ```"] pub fn leak (self) -> & 'static StaticExecutor { let ptr = self . state . load (Ordering :: Relaxed) ; let state : & 'static State = if ptr . is_null () { Box :: leak (Box :: new (State :: new ())) } else { unsafe { & * ptr } } ; std :: mem :: forget (self) ; let mut active = state . active . lock () . unwrap_or_else (PoisonError :: into_inner) ; if ! active . is_empty () { for waker in active . drain () { waker . wake () ; } * active = Slab :: new () ; } let static_executor : & 'static StaticExecutor = unsafe { std :: mem :: transmute (state) } ; static_executor } }
};
}
