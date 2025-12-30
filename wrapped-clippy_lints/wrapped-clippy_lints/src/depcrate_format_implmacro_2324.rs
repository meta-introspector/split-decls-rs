// Generated macro for macro_2324 (macro)
macro_rules! Depcrate_format_implmacro_2324 {
() => {
// Module: crate::format_impl
// Provides: {"macro_2324"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `println`, `print`, `eprintln` or `eprint` in an"] # [doc = " implementation of a formatting trait."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using a print macro is likely unintentional since formatting traits"] # [doc = " should write to the `Formatter`, not stdout/stderr."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::fmt::{Display, Error, Formatter};"] # [doc = ""] # [doc = " struct S;"] # [doc = " impl Display for S {"] # [doc = "     fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {"] # [doc = "         println!(\"S\");"] # [doc = ""] # [doc = "         Ok(())"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::fmt::{Display, Error, Formatter};"] # [doc = ""] # [doc = " struct S;"] # [doc = " impl Display for S {"] # [doc = "     fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {"] # [doc = "         writeln!(f, \"S\");"] # [doc = ""] # [doc = "         Ok(())"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.61.0"] pub PRINT_IN_FORMAT_IMPL , suspicious , "use of a print macro in a formatting trait impl" }
};
}
