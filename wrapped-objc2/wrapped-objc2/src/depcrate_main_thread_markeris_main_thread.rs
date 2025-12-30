// Generated macro for is_main_thread (function)
macro_rules! Depcrate_main_thread_markeris_main_thread {
() => {
// Module: crate::main_thread_marker
// Provides: {"is_main_thread"}
// Dependencies: {}
# [doc = " Whether the current thread is the main thread."] # [inline] fn is_main_thread () -> bool { # [cfg (target_vendor = "apple")] { # [cfg_attr (not (feature = "std") , link (name = "c" , kind = "dylib"))] extern "C" { fn pthread_main_np () -> core :: ffi :: c_int ; } unsafe { pthread_main_np () == 1 } } # [cfg (not (target_vendor = "apple"))] { unsafe { crate :: msg_send ! [crate :: class ! (NSThread) , isMainThread] } } }
};
}
