// Generated macro for spawn_blocking (function)
macro_rules! Depcrate_executorspawn_blocking {
() => {
// Module: crate::executor
// Provides: {"spawn_blocking"}
// Dependencies: {}
# [doc = " Runs blocking code on a thread pool."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Read the contents of a file:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # async_global_executor::block_on(async {"] # [doc = " let contents = async_global_executor::spawn_blocking(|| std::fs::read_to_string(\"file.txt\")).await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] # [doc = ""] # [doc = " Spawn a process:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::process::Command;"] # [doc = ""] # [doc = " # async_global_executor::block_on(async {"] # [doc = " let out = async_global_executor::spawn_blocking(|| Command::new(\"dir\").output()).await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub fn spawn_blocking < F : FnOnce () -> T + Send + 'static , T : Send + 'static > (f : F) -> Task < T > { blocking :: unblock (f) }
};
}
