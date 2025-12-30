// Generated macro for Injector (struct)
macro_rules! Depcrate_dequeInjector {
() => {
// Module: crate::deque
// Provides: {"Injector"}
// Dependencies: {}
# [doc = " An injector queue."] # [doc = ""] # [doc = " This is a FIFO queue that can be shared among multiple threads. Task schedulers typically have"] # [doc = " a single injector queue, which is the entry point for new tasks."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_deque::{Injector, Steal};"] # [doc = ""] # [doc = " let q = Injector::new();"] # [doc = " q.push(1);"] # [doc = " q.push(2);"] # [doc = ""] # [doc = " assert_eq!(q.steal(), Steal::Success(1));"] # [doc = " assert_eq!(q.steal(), Steal::Success(2));"] # [doc = " assert_eq!(q.steal(), Steal::Empty);"] # [doc = " ```"] pub struct Injector < T > { # [doc = " The head of the queue."] head : CachePadded < Position < T > > , # [doc = " The tail of the queue."] tail : CachePadded < Position < T > > , # [doc = " Indicates that dropping a `Injector<T>` may drop values of type `T`."] _marker : PhantomData < T > , }
};
}
