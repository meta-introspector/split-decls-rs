// Generated macro for impl_51 (impl)
macro_rules! Depcrateimpl_51 {
() => {
// Module: crate
// Provides: {"impl_51"}
// Dependencies: {}
impl Reaper { # [doc = " Get the singleton instance of the reaper."] fn get () -> & 'static Self { static REAPER : OnceLock < Reaper > = OnceLock :: new () ; REAPER . get_or_init (| | Reaper { sys : reaper :: Reaper :: new () , drivers : AtomicUsize :: new (0) , child_count : AtomicUsize :: new (0) , }) } # [doc = " Ensure that the reaper is driven."] # [doc = ""] # [doc = " If there are no active `driver()` callers, this will spawn the `async-process` thread."] # [inline] fn ensure_driven (& 'static self) { if self . drivers . compare_exchange (0 , 1 , Ordering :: SeqCst , Ordering :: Acquire) . is_ok () { self . start_driver_thread () ; } } # [doc = " Start the `async-process` thread."] # [cold] fn start_driver_thread (& 'static self) { # [cfg (test)] DRIVER_THREAD_SPAWNED . compare_exchange (false , true , Ordering :: SeqCst , Ordering :: SeqCst) . unwrap_or_else (| _ | unreachable ! ("Driver thread already spawned")) ; thread :: Builder :: new () . name ("async-process" . to_string ()) . spawn (move | | { let driver = async move { let guard = self . sys . lock () . await ; self . sys . reap (guard) . await } ; # [cfg (unix)] async_io :: block_on (driver) ; # [cfg (not (unix))] future :: block_on (driver) ; }) . expect ("cannot spawn async-process thread") ; } # [doc = " Register a process with this reaper."] fn register (& 'static self , child : std :: process :: Child) -> io :: Result < reaper :: ChildGuard > { self . ensure_driven () ; self . sys . register (child) } }
};
}
