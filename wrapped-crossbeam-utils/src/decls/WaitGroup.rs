macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! WaitGroup {
    () => {
        deps!();
        # [doc = " Enables threads to synchronize the beginning or end of some computation."] # [doc = ""] # [doc = " # Wait groups vs barriers"] # [doc = ""] # [doc = " `WaitGroup` is very similar to [`Barrier`], but there are a few differences:"] # [doc = ""] # [doc = " * [`Barrier`] needs to know the number of threads at construction, while `WaitGroup` is cloned to"] # [doc = "   register more threads."] # [doc = ""] # [doc = " * A [`Barrier`] can be reused even after all threads have synchronized, while a `WaitGroup`"] # [doc = "   synchronizes threads only once."] # [doc = ""] # [doc = " * All threads wait for others to reach the [`Barrier`]. With `WaitGroup`, each thread can choose"] # [doc = "   to either wait for other threads or to continue without blocking."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::sync::WaitGroup;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " // Create a new wait group."] # [doc = " let wg = WaitGroup::new();"] # [doc = ""] # [doc = " for _ in 0..4 {"] # [doc = "     // Create another reference to the wait group."] # [doc = "     let wg = wg.clone();"] # [doc = ""] # [doc = "     thread::spawn(move || {"] # [doc = "         // Do some work."] # [doc = ""] # [doc = "         // Drop the reference to the wait group."] # [doc = "         drop(wg);"] # [doc = "     });"] # [doc = " }"] # [doc = ""] # [doc = " // Block until all threads have finished their work."] # [doc = " wg.wait();"] # [doc = " # if cfg!(miri) { std::thread::sleep(std::time::Duration::from_millis(500)); } // wait for background threads closed: https://github.com/rust-lang/miri/issues/1371"] # [doc = " ```"] # [doc = ""] # [doc = " [`Barrier`]: std::sync::Barrier"] pub struct WaitGroup { inner : Arc < Inner > , }
    };
}

WaitGroup!()