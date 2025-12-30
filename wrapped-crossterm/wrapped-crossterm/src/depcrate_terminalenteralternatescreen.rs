// Generated macro for EnterAlternateScreen (struct)
macro_rules! Depcrate_terminalEnterAlternateScreen {
() => {
// Module: crate::terminal
// Provides: {"EnterAlternateScreen"}
// Dependencies: {}
# [doc = " A command that switches to alternate screen."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [doc = " * Use [LeaveAlternateScreen](./struct.LeaveAlternateScreen.html) command to leave the entered alternate screen."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::{self, Write};"] # [doc = " use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};"] # [doc = ""] # [doc = " fn main() -> io::Result<()> {"] # [doc = "     execute!(io::stdout(), EnterAlternateScreen)?;"] # [doc = ""] # [doc = "     // Do anything on the alternate screen"] # [doc = ""] # [doc = "     execute!(io::stdout(), LeaveAlternateScreen)"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct EnterAlternateScreen ;
};
}
