// Generated macro for macro_7100 (macro)
macro_rules! Depcrate_methodsmacro_7100 {
() => {
// Module: crate::methods
// Provides: {"macro_7100"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for calls to `Read::bytes` on types which don't implement `BufRead`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The default implementation calls `read` for each byte, which can be very inefficient for data that’s not in memory, such as `File`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::io::Read;"] # [doc = " use std::fs::File;"] # [doc = " let file = File::open(\"./bytes.txt\").unwrap();"] # [doc = " file.bytes();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::io::{BufReader, Read};"] # [doc = " use std::fs::File;"] # [doc = " let file = BufReader::new(File::open(\"./bytes.txt\").unwrap());"] # [doc = " file.bytes();"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub UNBUFFERED_BYTES , perf , "calling .bytes() is very inefficient when data is not in memory" }
};
}
