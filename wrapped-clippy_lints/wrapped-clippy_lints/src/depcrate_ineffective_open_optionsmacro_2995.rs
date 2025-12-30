// Generated macro for macro_2995 (macro)
macro_rules! Depcrate_ineffective_open_optionsmacro_2995 {
() => {
// Module: crate::ineffective_open_options
// Provides: {"macro_2995"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks if both `.write(true)` and `.append(true)` methods are called"] # [doc = " on a same `OpenOptions`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.append(true)` already enables `write(true)`, making this one"] # [doc = " superfluous."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::fs::OpenOptions;"] # [doc = " let _ = OpenOptions::new()"] # [doc = "            .write(true)"] # [doc = "            .append(true)"] # [doc = "            .create(true)"] # [doc = "            .open(\"file.json\");"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::fs::OpenOptions;"] # [doc = " let _ = OpenOptions::new()"] # [doc = "            .append(true)"] # [doc = "            .create(true)"] # [doc = "            .open(\"file.json\");"] # [doc = " ```"] # [clippy :: version = "1.76.0"] pub INEFFECTIVE_OPEN_OPTIONS , suspicious , "usage of both `write(true)` and `append(true)` on same `OpenOptions`" }
};
}
