// Generated macro for MoveUp (struct)
macro_rules! Depcrate_cursorMoveUp {
() => {
// Module: crate::cursor
// Provides: {"MoveUp"}
// Dependencies: {}
# [doc = " A command that moves the terminal cursor a given number of rows up."] # [doc = ""] # [doc = " # Notes"] # [doc = " * This command is 1 based, meaning `MoveUp(1)` moves the cursor up one cell."] # [doc = " * Most terminals default 0 argument to 1."] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MoveUp (pub u16) ;
};
}
