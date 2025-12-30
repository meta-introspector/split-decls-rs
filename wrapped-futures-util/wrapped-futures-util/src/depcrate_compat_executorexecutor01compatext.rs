// Generated macro for Executor01CompatExt (trait)
macro_rules! Depcrate_compat_executorExecutor01CompatExt {
() => {
// Module: crate::compat::executor
// Provides: {"Executor01CompatExt"}
// Dependencies: {}
# [doc = " Extension trait for futures 0.1 [`Executor`](futures_01::future::Executor)."] pub trait Executor01CompatExt : Executor01 < Executor01Future > + Clone + Send + 'static { # [doc = " Converts a futures 0.1 [`Executor`](futures_01::future::Executor) into a"] # [doc = " futures 0.3 [`Spawn`](futures_task::Spawn)."] # [doc = ""] # [doc = " ```"] # [doc = " # if cfg!(miri) { return; } // Miri does not support epoll"] # [doc = " use futures::task::SpawnExt;"] # [doc = " use futures::future::{FutureExt, TryFutureExt};"] # [doc = " use futures_util::compat::Executor01CompatExt;"] # [doc = " use tokio::executor::DefaultExecutor;"] # [doc = ""] # [doc = " # let (tx, rx) = futures::channel::oneshot::channel();"] # [doc = ""] # [doc = " let spawner = DefaultExecutor::current().compat();"] # [doc = " let future03 = async move {"] # [doc = "     println!(\"Running on the pool\");"] # [doc = "     spawner.spawn(async {"] # [doc = "         println!(\"Spawned!\");"] # [doc = "         # tx.send(42).unwrap();"] # [doc = "     }).unwrap();"] # [doc = " };"] # [doc = ""] # [doc = " let future01 = future03.unit_error().boxed().compat();"] # [doc = ""] # [doc = " tokio::run(future01);"] # [doc = " # futures::executor::block_on(rx).unwrap();"] # [doc = " ```"] fn compat (self) -> Executor01As03 < Self > where Self : Sized ; }
};
}
