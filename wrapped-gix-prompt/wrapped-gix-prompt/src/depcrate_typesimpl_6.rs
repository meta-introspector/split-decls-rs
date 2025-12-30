// Generated macro for impl_6 (impl)
macro_rules! Depcrate_typesimpl_6 {
() => {
// Module: crate::types
// Provides: {"impl_6"}
// Dependencies: {}
impl Options < '_ > { # [doc = " Change this instance to incorporate information from the environment."] # [doc = ""] # [doc = " - if `use_git_askpass` is true, use `GIT_ASKPASS` to override any existing [`askpass`][Options::askpass] program"] # [doc = " - otherwise fall back to the [`askpass`][Options::askpass] program already set"] # [doc = " - or try to use the `SSH_ASKPASS` if `use_ssh_askpass` is true"] # [doc = ""] # [doc = " At the and of this process, the `askpass` program may be set depending on the rules above."] # [doc = ""] # [doc = " Lastly, if `use_git_terminal_prompt` is set, use the `GIT_TERMINAL_PROMPT` environment variable and evaluate it as boolean,"] # [doc = " and if false, set [`mode`][Options::mode] to `disable`."] pub fn apply_environment (mut self , use_git_askpass : bool , use_ssh_askpass : bool , use_git_terminal_prompt : bool ,) -> Self { if let Some (askpass) = use_git_askpass . then (| | std :: env :: var_os ("GIT_ASKPASS")) . flatten () { self . askpass = Some (Cow :: Owned (askpass . into ())) ; } if self . askpass . is_none () { if let Some (askpass) = use_ssh_askpass . then (| | std :: env :: var_os ("SSH_ASKPASS")) . flatten () { self . askpass = Some (Cow :: Owned (askpass . into ())) ; } } self . mode = use_git_terminal_prompt . then (| | { std :: env :: var_os ("GIT_TERMINAL_PROMPT") . and_then (| val | gix_config_value :: Boolean :: try_from (val) . ok ()) . and_then (| allow | (! allow . 0) . then_some (Mode :: Disable)) }) . flatten () . unwrap_or (self . mode) ; self } }
};
}
