// Generated macro for MoveRight (struct)
macro_rules! Depcrate_cursorMoveRight {
() => {
// Module: crate::cursor
// Provides: {"MoveRight"}
// Dependencies: {}
# [doc = " A command that moves the terminal cursor a given number of columns to the right."] # [doc = ""] # [doc = " # Notes"] # [doc = " * This command is 1 based, meaning `MoveRight(1)` moves the cursor right one cell."] # [doc = " * Most terminals default 0 argument to 1."] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MoveRight (pub u16) ;
};
}
