// Generated macro for LocalExecutor (struct)
macro_rules! DepcrateLocalExecutor {
() => {
// Module: crate
// Provides: {"LocalExecutor"}
// Dependencies: {}
# [doc = " A thread-local executor."] # [doc = ""] # [doc = " The executor can only be run on the thread that created it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use async_executor::LocalExecutor;"] # [doc = " use futures_lite::future;"] # [doc = ""] # [doc = " let local_ex = LocalExecutor::new();"] # [doc = ""] # [doc = " future::block_on(local_ex.run(async {"] # [doc = "     println!(\"Hello world!\");"] # [doc = " }));"] # [doc = " ```"] pub struct LocalExecutor < 'a > { # [doc = " The inner executor."] inner : Executor < 'a > , # [doc = " Makes the type `!Send` and `!Sync`."] _marker : PhantomData < Rc < () > > , }
};
}
