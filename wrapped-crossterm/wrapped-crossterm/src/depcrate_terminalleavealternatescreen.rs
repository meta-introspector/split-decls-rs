// Generated macro for LeaveAlternateScreen (struct)
macro_rules! Depcrate_terminalLeaveAlternateScreen {
() => {
// Module: crate::terminal
// Provides: {"LeaveAlternateScreen"}
// Dependencies: {}
# [doc = " A command that switches back to the main screen."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [doc = " * Use [EnterAlternateScreen](./struct.EnterAlternateScreen.html) to enter the alternate screen."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::io::{self, Write};"] # [doc = " use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen}};"] # [doc = ""] # [doc = " fn main() -> io::Result<()> {"] # [doc = "     execute!(io::stdout(), EnterAlternateScreen)?;"] # [doc = ""] # [doc = "     // Do anything on the alternate screen"] # [doc = ""] # [doc = "     execute!(io::stdout(), LeaveAlternateScreen)"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct LeaveAlternateScreen ;
};
}
