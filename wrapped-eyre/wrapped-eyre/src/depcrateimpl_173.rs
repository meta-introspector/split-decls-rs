// Generated macro for impl_173 (impl)
macro_rules! Depcrateimpl_173 {
() => {
// Module: crate
// Provides: {"impl_173"}
// Dependencies: {}
impl DefaultHandler { # [doc = " Manual hook which constructs `DefaultHandler`s."] # [doc = ""] # [doc = " # Details"] # [doc = ""] # [doc = " When supplied to the `set_hook` function, `default_with` will cause `eyre::Report` to use"] # [doc = " `DefaultHandler` as the error report handler."] # [doc = ""] # [doc = " If the `auto-install` feature is enabled, and a user-provided hook for constructing"] # [doc = " `EyreHandlers` was not installed using `set_hook`, `DefaultHandler::default_with`"] # [doc = " is automatically installed as the hook."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,should_panic"] # [doc = " use eyre::{DefaultHandler, eyre, InstallError, Result, set_hook};"] # [doc = ""] # [doc = " fn main() -> Result<()> {"] # [doc = "     install_default().expect(\"default handler inexplicably already installed\");"] # [doc = "     Err(eyre!(\"hello from default error city!\"))"] # [doc = " }"] # [doc = ""] # [doc = " fn install_default() -> Result<(), InstallError> {"] # [doc = "     set_hook(Box::new(DefaultHandler::default_with))"] # [doc = " }"] # [doc = ""] # [doc = " ```"] # [allow (unused_variables)] # [cfg_attr (not (feature = "auto-install") , allow (dead_code))] pub fn default_with (error : & (dyn StdError + 'static)) -> Box < dyn EyreHandler > { let backtrace = backtrace_if_absent ! (error) ; Box :: new (Self { backtrace , # [cfg (track_caller)] location : None , }) } }
};
}
