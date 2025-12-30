// Generated macro for macro_10563 (macro)
macro_rules! Depcrate_unnecessary_self_importsmacro_10563 {
() => {
// Module: crate::unnecessary_self_imports
// Provides: {"macro_10563"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for imports ending in `::{self}`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " In most cases, this can be written much more cleanly by omitting `::{self}`."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Removing `::{self}` will cause any non-module items at the same path to also be imported."] # [doc = " This might cause a naming conflict (https://github.com/rust-lang/rustfmt/issues/3568). This lint makes no attempt"] # [doc = " to detect this scenario and that is why it is a restriction lint."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::io::{self};"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::io;"] # [doc = " ```"] # [clippy :: version = "1.53.0"] pub UNNECESSARY_SELF_IMPORTS , restriction , "imports ending in `::{self}`, which can be omitted" }
};
}
