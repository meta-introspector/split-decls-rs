// Generated macro for MoveLeft (struct)
macro_rules! Depcrate_cursorMoveLeft {
() => {
// Module: crate::cursor
// Provides: {"MoveLeft"}
// Dependencies: {}
# [doc = " A command that moves the terminal cursor a given number of columns to the left."] # [doc = ""] # [doc = " # Notes"] # [doc = " * This command is 1 based, meaning `MoveLeft(1)` moves the cursor left one cell."] # [doc = " * Most terminals default 0 argument to 1."] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MoveLeft (pub u16) ;
};
}
