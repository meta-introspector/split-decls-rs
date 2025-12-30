// Generated macro for run_on_main (function)
macro_rules! Depcrate_main_thread_boundrun_on_main {
() => {
// Module: crate::main_thread_bound
// Provides: {"run_on_main"}
// Dependencies: {}
# [doc = " Submit the given closure to the runloop on the main thread."] # [doc = ""] # [doc = " If the current thread is the main thread, this runs the closure."] # [doc = ""] # [doc = " The closure is passed a [`MainThreadMarker`] that it can further use"] # [doc = " to access APIs that are only accessible from the main thread."] # [doc = ""] # [doc = " This function should only be used in applications whose main thread is"] # [doc = " running an event loop with `dispatch_main`, `UIApplicationMain`,"] # [doc = " `NSApplicationMain`, `CFRunLoop` or similar; it will block"] # [doc = " indefinitely if that is not the case."] # [doc = ""] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use dispatch2::run_on_main;"] # [doc = " run_on_main(|mtm| {"] # [doc = "     // Do something on the main thread with the given marker"] # [doc = " });"] # [doc = " ```"] pub fn run_on_main < F , R > (f : F) -> R where F : Send + FnOnce (MainThreadMarker) -> R , R : Send , { if let Some (mtm) = MainThreadMarker :: new () { f (mtm) } else { let mut ret = None ; DispatchQueue :: main () . exec_sync (| | { ret = Some (f (unsafe { MainThreadMarker :: new_unchecked () })) }) ; ret . unwrap () } }
};
}
