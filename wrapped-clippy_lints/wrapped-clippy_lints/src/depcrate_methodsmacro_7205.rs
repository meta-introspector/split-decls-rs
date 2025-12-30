// Generated macro for macro_7205 (macro)
macro_rules! Depcrate_methodsmacro_7205 {
() => {
// Module: crate::methods
// Provides: {"macro_7205"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks if the `seek` method of the `Seek` trait is called with `SeekFrom::Current(0)`,"] # [doc = " and if it is, suggests using `stream_position` instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Readability. Use dedicated method."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " use std::fs::File;"] # [doc = " use std::io::{self, Write, Seek, SeekFrom};"] # [doc = ""] # [doc = " fn main() -> io::Result<()> {"] # [doc = "     let mut f = File::create(\"foo.txt\")?;"] # [doc = "     f.write_all(b\"Hello\")?;"] # [doc = "     eprintln!(\"Written {} bytes\", f.seek(SeekFrom::Current(0))?);"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,no_run"] # [doc = " use std::fs::File;"] # [doc = " use std::io::{self, Write, Seek, SeekFrom};"] # [doc = ""] # [doc = " fn main() -> io::Result<()> {"] # [doc = "     let mut f = File::create(\"foo.txt\")?;"] # [doc = "     f.write_all(b\"Hello\")?;"] # [doc = "     eprintln!(\"Written {} bytes\", f.stream_position()?);"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.67.0"] pub SEEK_FROM_CURRENT , complexity , "use dedicated method for seek from current position" }
};
}
