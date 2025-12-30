// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
impl ChildStdout { # [doc = " Convert async_process::ChildStdout into std::process::Stdio."] # [doc = ""] # [doc = " You can use it to associate to the next process."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_process::Command;"] # [doc = " use std::process::Stdio;"] # [doc = " use std::io::Read;"] # [doc = " use futures_lite::AsyncReadExt;"] # [doc = ""] # [doc = " let mut ls_child = Command::new(\"ls\").stdout(Stdio::piped()).spawn()?;"] # [doc = " let stdio:Stdio = ls_child.stdout.take().unwrap().into_stdio().await?;"] # [doc = ""] # [doc = " let mut echo_child = Command::new(\"echo\").stdin(stdio).stdout(Stdio::piped()).spawn()?;"] # [doc = " let mut buf = vec![];"] # [doc = " echo_child.stdout.take().unwrap().read(&mut buf).await;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub async fn into_stdio (self) -> io :: Result < std :: process :: Stdio > { cfg_if :: cfg_if ! { if # [cfg (windows)] { Ok (self . 0 . into_inner () . await . into ()) } else if # [cfg (unix)] { let child_stdout = self . 0 . into_inner () ?; blocking_fd (rustix :: fd :: AsFd :: as_fd (& child_stdout)) ?; Ok (child_stdout . into ()) } } } }
};
}
