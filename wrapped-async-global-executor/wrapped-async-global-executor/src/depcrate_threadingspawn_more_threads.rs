// Generated macro for spawn_more_threads (function)
macro_rules! Depcrate_threadingspawn_more_threads {
() => {
// Module: crate::threading
// Provides: {"spawn_more_threads"}
// Dependencies: {}
# [doc = " Spawn more executor threads, up to configured max value."] # [doc = ""] # [doc = " Returns how many threads we spawned."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " async_global_executor::spawn_more_threads(2);"] # [doc = " ```"] pub async fn spawn_more_threads (count : usize) -> io :: Result < usize > { let config = crate :: config :: GLOBAL_EXECUTOR_CONFIG . get () . unwrap_or_else (| | { crate :: init () ; crate :: config :: GLOBAL_EXECUTOR_CONFIG . get () . unwrap () }) ; let mut threads_number = GLOBAL_EXECUTOR_THREADS_NUMBER . lock () . await ; let mut expected_threads_number = GLOBAL_EXECUTOR_EXPECTED_THREADS_NUMBER . lock () . await ; let count = count . min (config . max_threads - * threads_number) ; for _ in 0 .. count { thread :: Builder :: new () . name ((config . thread_name_fn) ()) . spawn (thread_main_loop) ? ; * threads_number += 1 ; * expected_threads_number += 1 ; } Ok (count) }
};
}
