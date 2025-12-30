// Generated macro for MoveToNextLine (struct)
macro_rules! Depcrate_cursorMoveToNextLine {
() => {
// Module: crate::cursor
// Provides: {"MoveToNextLine"}
// Dependencies: {}
# [doc = " A command that moves the terminal cursor down the given number of lines,"] # [doc = " and moves it to the first column."] # [doc = ""] # [doc = " # Notes"] # [doc = " * This command is 1 based, meaning `MoveToNextLine(1)` moves to the next line."] # [doc = " * Most terminals default 0 argument to 1."] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MoveToNextLine (pub u16) ;
};
}
