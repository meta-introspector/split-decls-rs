// Generated macro for macro_7203 (macro)
macro_rules! Depcrate_methodsmacro_7203 {
() => {
// Module: crate::methods
// Provides: {"macro_7203"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of File::read_to_end and File::read_to_string."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " `fs::{read, read_to_string}` provide the same functionality when `buf` is empty with fewer imports and no intermediate values."] # [doc = " See also: [fs::read docs](https://doc.rust-lang.org/std/fs/fn.read.html), [fs::read_to_string docs](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,no_run"] # [doc = " # use std::io::Read;"] # [doc = " # use std::fs::File;"] # [doc = " let mut f = File::open(\"foo.txt\").unwrap();"] # [doc = " let mut bytes = Vec::new();"] # [doc = " f.read_to_end(&mut bytes).unwrap();"] # [doc = " ```"] # [doc = " Can be written more concisely as"] # [doc = " ```rust,no_run"] # [doc = " # use std::fs;"] # [doc = " let mut bytes = fs::read(\"foo.txt\").unwrap();"] # [doc = " ```"] # [clippy :: version = "1.44.0"] pub VERBOSE_FILE_READS , restriction , "use of `File::read_to_end` or `File::read_to_string`" }
};
}
