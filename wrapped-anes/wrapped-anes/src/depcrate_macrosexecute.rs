// Generated macro for execute (macro)
macro_rules! Depcrate_macrosexecute {
() => {
// Module: crate::macros
// Provides: {"execute"}
// Dependencies: {}
# [doc = " Executes ANSI escape sequence(s)."] # [doc = ""] # [doc = " What does execute mean exactly? All sequences are queued with the"] # [doc = " `write!($dst, \"{}\", $sequence)` macro and then the"] # [doc = " [`flush`](https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush) method"] # [doc = " is called."] # [doc = ""] # [doc = " Check the [`queue!`](macro.queue.html) macro if you'd like queue sequences"] # [doc = " and execute them later."] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::{Result, Write};"] # [doc = ""] # [doc = " use anes::execute;"] # [doc = ""] # [doc = " fn main() -> Result<()> {"] # [doc = "     let mut stdout = std::io::stdout();"] # [doc = "     execute!("] # [doc = "         &mut stdout,"] # [doc = "         anes::SaveCursorPosition,"] # [doc = "         anes::MoveCursorTo(10, 10),"] # [doc = "         anes::RestoreCursorPosition"] # [doc = "     )?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! execute { ($ dst : expr_2021 , $ ($ sequence : expr_2021) ,* $ (,) ?) => { { if let Err (e) = $ crate :: queue ! ($ dst , $ ($ sequence) ,*) { Err (e) } else { $ dst . flush () } } } }
};
}
