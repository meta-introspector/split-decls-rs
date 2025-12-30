// Generated macro for macro_7568 (macro)
macro_rules! Depcrate_mut_referencemacro_7568 {
() => {
// Module: crate::mut_reference
// Provides: {"macro_7568"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects passing a mutable reference to a function that only"] # [doc = " requires an immutable reference."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The mutable reference rules out all other references to"] # [doc = " the value. Also the code misleads about the intent of the call site."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let mut vec = Vec::new();"] # [doc = " # let mut value = 5;"] # [doc = " vec.push(&mut value);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let mut vec = Vec::new();"] # [doc = " # let value = 5;"] # [doc = " vec.push(&value);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub UNNECESSARY_MUT_PASSED , style , "an argument passed as a mutable reference although the callee only demands an immutable reference" }
};
}
