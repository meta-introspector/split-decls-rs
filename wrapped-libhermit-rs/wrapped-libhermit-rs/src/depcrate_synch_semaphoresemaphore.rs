// Generated macro for Semaphore (struct)
macro_rules! Depcrate_synch_semaphoreSemaphore {
() => {
// Module: crate::synch::semaphore
// Provides: {"Semaphore"}
// Dependencies: {}
# [doc = " A counting, blocking, semaphore."] # [doc = ""] # [doc = " Semaphores are a form of atomic counter where access is only granted if the"] # [doc = " counter is a positive value. Each acquisition will block the calling thread"] # [doc = " until the counter is positive, and each release will increment the counter"] # [doc = " and unblock any threads if necessary."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = ""] # [doc = " // Create a semaphore that represents 5 resources"] # [doc = " let sem = Semaphore::new(5);"] # [doc = ""] # [doc = " // Acquire one of the resources"] # [doc = " sem.acquire();"] # [doc = ""] # [doc = " // Acquire one of the resources for a limited period of time"] # [doc = " {"] # [doc = "     let _guard = sem.access();"] # [doc = "     // ..."] # [doc = " } // resources is released here"] # [doc = ""] # [doc = " // Release our initially acquired resource"] # [doc = " sem.release();"] # [doc = ""] # [doc = " Interface is derived from https://doc.rust-lang.org/1.7.0/src/std/sync/semaphore.rs.html"] # [doc = " ```"] pub struct Semaphore { state : InterruptTicketMutex < SemaphoreState > , }
};
}
