// Generated macro for queue (macro)
macro_rules! Depcrate_macrosqueue {
() => {
// Module: crate::macros
// Provides: {"queue"}
// Dependencies: {}
# [doc = " Queues ANSI escape sequence(s)."] # [doc = ""] # [doc = " What does queue mean exactly? All sequences are queued with the"] # [doc = " `write!($dst, \"{}\", $sequence)` macro without calling the"] # [doc = " [`flush`](https://doc.rust-lang.org/std/io/trait.Write.html#tymethod.flush) method."] # [doc = ""] # [doc = " Check the [`execute!`](macro.execute.html) macro if you'd like execute them"] # [doc = " immediately (call the `flush` method after all sequences were queued)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::{Result, Write};"] # [doc = ""] # [doc = " use anes::queue;"] # [doc = ""] # [doc = " fn main() -> Result<()> {"] # [doc = "     let mut stdout = std::io::stdout();"] # [doc = "     queue!("] # [doc = "         &mut stdout,"] # [doc = "         anes::SaveCursorPosition,"] # [doc = "         anes::MoveCursorTo(10, 10)"] # [doc = "     )?;"] # [doc = ""] # [doc = "     queue!(&mut stdout, anes::RestoreCursorPosition,)?;"] # [doc = ""] # [doc = "     // ANSI sequences are not executed until you flush it!"] # [doc = "     stdout.flush()"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! queue { ($ dst : expr_2021 , $ ($ sequence : expr_2021) ,* $ (,) ?) => { { let mut error = None ; $ (if let Err (e) = write ! ($ dst , "{}" , $ sequence) { error = Some (e) ; }) * if let Some (error) = error { Err (error) } else { Ok (()) } } } }
};
}
