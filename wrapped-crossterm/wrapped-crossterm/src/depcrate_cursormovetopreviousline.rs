// Generated macro for MoveToPreviousLine (struct)
macro_rules! Depcrate_cursorMoveToPreviousLine {
() => {
// Module: crate::cursor
// Provides: {"MoveToPreviousLine"}
// Dependencies: {}
# [doc = " A command that moves the terminal cursor up the given number of lines,"] # [doc = " and moves it to the first column."] # [doc = ""] # [doc = " # Notes"] # [doc = " * This command is 1 based, meaning `MoveToPreviousLine(1)` moves to the previous line."] # [doc = " * Most terminals default 0 argument to 1."] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MoveToPreviousLine (pub u16) ;
};
}
