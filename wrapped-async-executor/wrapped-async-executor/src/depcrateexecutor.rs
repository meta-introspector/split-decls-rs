// Generated macro for Executor (struct)
macro_rules! DepcrateExecutor {
() => {
// Module: crate
// Provides: {"Executor"}
// Dependencies: {}
# [doc = " An async executor."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " A multi-threaded executor:"] # [doc = ""] # [doc = " ```"] # [doc = " use async_channel::unbounded;"] # [doc = " use async_executor::Executor;"] # [doc = " use easy_parallel::Parallel;"] # [doc = " use futures_lite::future;"] # [doc = ""] # [doc = " let ex = Executor::new();"] # [doc = " let (signal, shutdown) = unbounded::<()>();"] # [doc = ""] # [doc = " Parallel::new()"] # [doc = "     // Run four executor threads."] # [doc = "     .each(0..4, |_| future::block_on(ex.run(shutdown.recv())))"] # [doc = "     // Run the main future on the current thread."] # [doc = "     .finish(|| future::block_on(async {"] # [doc = "         println!(\"Hello world!\");"] # [doc = "         drop(signal);"] # [doc = "     }));"] # [doc = " ```"] pub struct Executor < 'a > { # [doc = " The executor state."] pub (crate) state : AtomicPtr < State > , # [doc = " Makes the `'a` lifetime invariant."] _marker : PhantomData < std :: cell :: UnsafeCell < & 'a () > > , }
};
}
