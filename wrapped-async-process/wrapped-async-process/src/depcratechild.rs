// Generated macro for Child (struct)
macro_rules! DepcrateChild {
() => {
// Module: crate
// Provides: {"Child"}
// Dependencies: {}
# [doc = " A spawned child process."] # [doc = ""] # [doc = " The process can be in running or exited state. Use [`status()`][`Child::status()`] or"] # [doc = " [`output()`][`Child::output()`] to wait for it to exit."] # [doc = ""] # [doc = " If the [`Child`] is dropped, the process keeps running in the background."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Spawn a process and wait for it to complete:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_process::Command;"] # [doc = ""] # [doc = " Command::new(\"cp\").arg(\"a.txt\").arg(\"b.txt\").status().await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub struct Child { # [doc = " The handle for writing to the child's standard input (stdin), if it has been captured."] pub stdin : Option < ChildStdin > , # [doc = " The handle for reading from the child's standard output (stdout), if it has been captured."] pub stdout : Option < ChildStdout > , # [doc = " The handle for reading from the child's standard error (stderr), if it has been captured."] pub stderr : Option < ChildStderr > , # [doc = " The inner child process handle."] child : Arc < Mutex < ChildGuard > > , }
};
}
