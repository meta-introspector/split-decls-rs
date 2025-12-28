macro_rules! deps {
    () => {
        Inner!();
        Buffer!();
        Flavor!();
    };
}

macro_rules! Worker {
    () => {
        deps!();
        # [doc = " A worker queue."] # [doc = ""] # [doc = " This is a FIFO or LIFO queue that is owned by a single thread, but other threads may steal"] # [doc = " tasks from it. Task schedulers typically create a single worker queue per thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " A FIFO worker:"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_deque::{Steal, Worker};"] # [doc = ""] # [doc = " let w = Worker::new_fifo();"] # [doc = " let s = w.stealer();"] # [doc = ""] # [doc = " w.push(1);"] # [doc = " w.push(2);"] # [doc = " w.push(3);"] # [doc = ""] # [doc = " assert_eq!(s.steal(), Steal::Success(1));"] # [doc = " assert_eq!(w.pop(), Some(2));"] # [doc = " assert_eq!(w.pop(), Some(3));"] # [doc = " ```"] # [doc = ""] # [doc = " A LIFO worker:"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_deque::{Steal, Worker};"] # [doc = ""] # [doc = " let w = Worker::new_lifo();"] # [doc = " let s = w.stealer();"] # [doc = ""] # [doc = " w.push(1);"] # [doc = " w.push(2);"] # [doc = " w.push(3);"] # [doc = ""] # [doc = " assert_eq!(s.steal(), Steal::Success(1));"] # [doc = " assert_eq!(w.pop(), Some(3));"] # [doc = " assert_eq!(w.pop(), Some(2));"] # [doc = " ```"] pub struct Worker < T > { # [doc = " A reference to the inner representation of the queue."] inner : Arc < CachePadded < Inner < T > > > , # [doc = " A copy of `inner.buffer` for quick access."] buffer : Cell < Buffer < T > > , # [doc = " The flavor of the queue."] flavor : Flavor , # [doc = " Indicates that the worker cannot be shared among threads."] _marker : PhantomData < * mut () > , }
    };
}

Worker!();