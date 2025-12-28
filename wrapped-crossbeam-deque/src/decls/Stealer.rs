macro_rules! deps {
    () => {
        Flavor!();
        Worker!();
        Inner!();
    };
}

macro_rules! Stealer {
    () => {
        deps!();
        # [doc = " A stealer handle of a worker queue."] # [doc = ""] # [doc = " Stealers can be shared among threads."] # [doc = ""] # [doc = " Task schedulers typically have a single worker queue per worker thread."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_deque::{Steal, Worker};"] # [doc = ""] # [doc = " let w = Worker::new_lifo();"] # [doc = " w.push(1);"] # [doc = " w.push(2);"] # [doc = ""] # [doc = " let s = w.stealer();"] # [doc = " assert_eq!(s.steal(), Steal::Success(1));"] # [doc = " assert_eq!(s.steal(), Steal::Success(2));"] # [doc = " assert_eq!(s.steal(), Steal::Empty);"] # [doc = " ```"] pub struct Stealer < T > { # [doc = " A reference to the inner representation of the queue."] inner : Arc < CachePadded < Inner < T > > > , # [doc = " The flavor of the queue."] flavor : Flavor , }
    };
}

Stealer!()