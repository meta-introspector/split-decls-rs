// Generated macro for macro_7213 (macro)
macro_rules! Depcrate_methodsmacro_7213 {
() => {
// Module: crate::methods
// Provides: {"macro_7213"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Looks for calls to [`Stdin::read_line`] to read a line from the standard input"] # [doc = " into a string, then later attempting to use that string for an operation that will never"] # [doc = " work for strings with a trailing newline character in it (e.g. parsing into a `i32`)."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The operation will always fail at runtime no matter what the user enters, thus"] # [doc = " making it a useless operation."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " let mut input = String::new();"] # [doc = " std::io::stdin().read_line(&mut input).expect(\"Failed to read a line\");"] # [doc = " let num: i32 = input.parse().expect(\"Not a number!\");"] # [doc = " assert_eq!(num, 42); // we never even get here!"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " let mut input = String::new();"] # [doc = " std::io::stdin().read_line(&mut input).expect(\"Failed to read a line\");"] # [doc = " let num: i32 = input.trim_end().parse().expect(\"Not a number!\");"] # [doc = " //                  ^^^^^^^^^^^ remove the trailing newline"] # [doc = " assert_eq!(num, 42);"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub READ_LINE_WITHOUT_TRIM , correctness , "calling `Stdin::read_line`, then trying to parse it without first trimming" }
};
}
