// Generated macro for impl_20 (impl)
macro_rules! Depcrate_helper_cascadeimpl_20 {
() => {
// Module: crate::helper::cascade
// Provides: {"impl_20"}
// Dependencies: {}
# [doc = " Initialization"] impl Cascade { # [doc = " Return the programs to run for the current platform."] # [doc = ""] # [doc = " These are typically used as basis for all credential cascade invocations, with configured programs following afterwards."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " These defaults emulate what typical git installations may use these days, as in fact it's a configurable which comes"] # [doc = " from installation-specific configuration files which we cannot know (or guess at best)."] # [doc = " This seems like an acceptable trade-off as helpers are ignored if they fail or are not existing."] pub fn platform_builtin () -> Vec < Program > { if cfg ! (target_os = "macos") { Some ("osxkeychain") } else if cfg ! (target_os = "linux") { Some ("libsecret") } else if cfg ! (target_os = "windows") { Some ("manager-core") } else { None } . map (| name | vec ! [Program :: from_custom_definition (name)]) . unwrap_or_default () } }
};
}
