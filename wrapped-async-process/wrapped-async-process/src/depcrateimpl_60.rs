// Generated macro for impl_60 (impl)
macro_rules! Depcrateimpl_60 {
() => {
// Module: crate
// Provides: {"impl_60"}
// Dependencies: {}
impl ChildStdin { # [doc = " Convert async_process::ChildStdin into std::process::Stdio."] # [doc = ""] # [doc = " You can use it to associate to the next process."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_process::Command;"] # [doc = " use std::process::Stdio;"] # [doc = ""] # [doc = " let mut ls_child = Command::new(\"ls\").stdin(Stdio::piped()).spawn()?;"] # [doc = " let stdio:Stdio = ls_child.stdin.take().unwrap().into_stdio().await?;"] # [doc = ""] # [doc = " let mut echo_child = Command::new(\"echo\").arg(\"./\").stdout(stdio).spawn()?;"] # [doc = ""] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn into_stdio (self) -> io :: Result < std :: process :: Stdio > { cfg_if :: cfg_if ! { if # [cfg (windows)] { Ok (self . 0 . into_inner () . await . into ()) } else if # [cfg (unix)] { let child_stdin = self . 0 . into_inner () ?; blocking_fd (rustix :: fd :: AsFd :: as_fd (& child_stdin)) ?; Ok (child_stdin . into ()) } } } }
};
}
