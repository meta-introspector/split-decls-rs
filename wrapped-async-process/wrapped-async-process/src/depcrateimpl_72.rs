// Generated macro for impl_72 (impl)
macro_rules! Depcrateimpl_72 {
() => {
// Module: crate
// Provides: {"impl_72"}
// Dependencies: {}
impl ChildStderr { # [doc = " Convert async_process::ChildStderr into std::process::Stdio."] # [doc = ""] # [doc = " You can use it to associate to the next process."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_process::Command;"] # [doc = " use std::process::Stdio;"] # [doc = ""] # [doc = " let mut ls_child = Command::new(\"ls\").arg(\"x\").stderr(Stdio::piped()).spawn()?;"] # [doc = " let stdio:Stdio = ls_child.stderr.take().unwrap().into_stdio().await?;"] # [doc = ""] # [doc = " let mut echo_child = Command::new(\"echo\").stdin(stdio).spawn()?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn into_stdio (self) -> io :: Result < std :: process :: Stdio > { cfg_if :: cfg_if ! { if # [cfg (windows)] { Ok (self . 0 . into_inner () . await . into ()) } else if # [cfg (unix)] { let child_stderr = self . 0 . into_inner () ?; blocking_fd (rustix :: fd :: AsFd :: as_fd (& child_stderr)) ?; Ok (child_stderr . into ()) } } } }
};
}
