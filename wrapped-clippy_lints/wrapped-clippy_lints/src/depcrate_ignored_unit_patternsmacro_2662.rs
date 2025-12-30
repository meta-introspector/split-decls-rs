// Generated macro for macro_2662 (macro)
macro_rules! Depcrate_ignored_unit_patternsmacro_2662 {
() => {
// Module: crate::ignored_unit_patterns
// Provides: {"macro_2662"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_` in patterns of type `()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Matching with `()` explicitly instead of `_` outlines"] # [doc = " the fact that the pattern contains no data. Also it"] # [doc = " would detect a type change that `_` would ignore."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " match std::fs::create_dir(\"tmp-work-dir\") {"] # [doc = "     Ok(_) => println!(\"Working directory created\"),"] # [doc = "     Err(s) => eprintln!(\"Could not create directory: {s}\"),"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " match std::fs::create_dir(\"tmp-work-dir\") {"] # [doc = "     Ok(()) => println!(\"Working directory created\"),"] # [doc = "     Err(s) => eprintln!(\"Could not create directory: {s}\"),"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub IGNORED_UNIT_PATTERNS , pedantic , "suggest replacing `_` by `()` in patterns where appropriate" }
};
}
