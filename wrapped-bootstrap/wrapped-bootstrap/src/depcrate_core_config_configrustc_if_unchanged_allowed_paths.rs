// Generated macro for RUSTC_IF_UNCHANGED_ALLOWED_PATHS (const)
macro_rules! Depcrate_core_config_configRUSTC_IF_UNCHANGED_ALLOWED_PATHS {
() => {
// Module: crate::core::config::config
// Provides: {"RUSTC_IF_UNCHANGED_ALLOWED_PATHS"}
// Dependencies: {}
# [doc = " Each path in this list is considered \"allowed\" in the `download-rustc=\"if-unchanged\"` logic."] # [doc = " This means they can be modified and changes to these paths should never trigger a compiler build"] # [doc = " when \"if-unchanged\" is set."] # [doc = ""] # [doc = " NOTE: Paths must have the \":!\" prefix to tell git to ignore changes in those paths during"] # [doc = " the diff check."] # [doc = ""] # [doc = " WARNING: Be cautious when adding paths to this list. If a path that influences the compiler build"] # [doc = " is added here, it will cause bootstrap to skip necessary rebuilds, which may lead to risky results."] # [doc = " For example, \"src/bootstrap\" should never be included in this list as it plays a crucial role in the"] # [doc = " final output/compiler, which can be significantly affected by changes made to the bootstrap sources."] # [rustfmt :: skip] pub const RUSTC_IF_UNCHANGED_ALLOWED_PATHS : & [& str] = & [":!library" , ":!src/tools" , ":!src/librustdoc" , ":!src/rustdoc-json-types" , ":!tests" , ":!triagebot.toml" ,] ;
};
}
