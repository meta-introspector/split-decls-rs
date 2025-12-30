// Generated macro for Command (struct)
macro_rules! DepcrateCommand {
() => {
// Module: crate
// Provides: {"Command"}
// Dependencies: {}
# [doc = " A builder for spawning processes."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # futures_lite::future::block_on(async {"] # [doc = " use async_process::Command;"] # [doc = ""] # [doc = " let output = if cfg!(target_os = \"windows\") {"] # [doc = "     Command::new(\"cmd\").args(&[\"/C\", \"echo hello\"]).output().await?"] # [doc = " } else {"] # [doc = "     Command::new(\"sh\").arg(\"-c\").arg(\"echo hello\").output().await?"] # [doc = " };"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] pub struct Command { inner : std :: process :: Command , stdin : bool , stdout : bool , stderr : bool , reap_on_drop : bool , kill_on_drop : bool , }
};
}
