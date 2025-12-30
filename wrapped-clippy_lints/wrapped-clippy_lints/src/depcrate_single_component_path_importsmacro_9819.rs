// Generated macro for macro_9819 (macro)
macro_rules! Depcrate_single_component_path_importsmacro_9819 {
() => {
// Module: crate::single_component_path_imports
// Provides: {"macro_9819"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checking for imports with single component use path."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Import with single component use path such as `use cratename;`"] # [doc = " is not necessary, and thus should be removed."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " use regex;"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     regex::Regex::new(r\"^\\d{4}-\\d{2}-\\d{2}$\").unwrap();"] # [doc = " }"] # [doc = " ```"] # [doc = " Better as"] # [doc = " ```rust,ignore"] # [doc = " fn main() {"] # [doc = "     regex::Regex::new(r\"^\\d{4}-\\d{2}-\\d{2}$\").unwrap();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.43.0"] pub SINGLE_COMPONENT_PATH_IMPORTS , style , "imports with single component path are redundant" }
};
}
