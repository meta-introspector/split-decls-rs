// Generated macro for impl_195 (impl)
macro_rules! Depcrate_sync_wait_groupimpl_195 {
() => {
// Module: crate::sync::wait_group
// Provides: {"impl_195"}
// Dependencies: {}
impl WaitGroup { # [doc = " Creates a new wait group and returns the single reference to it."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::sync::WaitGroup;"] # [doc = ""] # [doc = " let wg = WaitGroup::new();"] # [doc = " ```"] pub fn new () -> Self { Self :: default () } # [doc = " Drops this reference and waits until all other references are dropped."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::sync::WaitGroup;"] # [doc = " use std::thread;"] # [doc = ""] # [doc = " let wg = WaitGroup::new();"] # [doc = ""] # [doc = " # let t ="] # [doc = " thread::spawn({"] # [doc = "     let wg = wg.clone();"] # [doc = "     move || {"] # [doc = "         // Block until both threads have reached `wait()`."] # [doc = "         wg.wait();"] # [doc = "     }"] # [doc = " });"] # [doc = ""] # [doc = " // Block until both threads have reached `wait()`."] # [doc = " wg.wait();"] # [doc = " # t.join().unwrap(); // join thread to avoid https://github.com/rust-lang/miri/issues/1371"] # [doc = " ```"] pub fn wait (self) { if * self . inner . count . lock () . unwrap () == 1 { return ; } let inner = self . inner . clone () ; drop (self) ; let mut count = inner . count . lock () . unwrap () ; while * count > 0 { count = inner . cvar . wait (count) . unwrap () ; } } }
};
}
