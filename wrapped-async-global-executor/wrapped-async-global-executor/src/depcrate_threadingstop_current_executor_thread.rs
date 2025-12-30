// Generated macro for stop_current_executor_thread (function)
macro_rules! Depcrate_threadingstop_current_executor_thread {
() => {
// Module: crate::threading
// Provides: {"stop_current_executor_thread"}
// Dependencies: {}
async fn stop_current_executor_thread () -> bool { let mut expected_threads_number = GLOBAL_EXECUTOR_EXPECTED_THREADS_NUMBER . lock () . await ; if * expected_threads_number > crate :: config :: GLOBAL_EXECUTOR_CONFIG . get () . unwrap () . min_threads { let (s , r_ack) = THREAD_SHUTDOWN . with (| thread_shutdown | thread_shutdown . get () . unwrap () . clone ()) ; let _ = s . send (()) . await ; * expected_threads_number -= 1 ; drop (expected_threads_number) ; let _ = r_ack . recv () . await ; * GLOBAL_EXECUTOR_THREADS_NUMBER . lock () . await -= 1 ; true } else { false } }
};
}
