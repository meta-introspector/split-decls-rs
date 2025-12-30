// Generated macro for MoveDown (struct)
macro_rules! Depcrate_cursorMoveDown {
() => {
// Module: crate::cursor
// Provides: {"MoveDown"}
// Dependencies: {}
# [doc = " A command that moves the terminal cursor a given number of rows down."] # [doc = ""] # [doc = " # Notes"] # [doc = " * This command is 1 based, meaning `MoveDown(1)` moves the cursor down one cell."] # [doc = " * Most terminals default 0 argument to 1."] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MoveDown (pub u16) ;
};
}
