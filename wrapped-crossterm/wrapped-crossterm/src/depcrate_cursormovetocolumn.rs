// Generated macro for MoveToColumn (struct)
macro_rules! Depcrate_cursorMoveToColumn {
() => {
// Module: crate::cursor
// Provides: {"MoveToColumn"}
// Dependencies: {}
# [doc = " A command that moves the terminal cursor to the given column on the current row."] # [doc = ""] # [doc = " # Notes"] # [doc = " * This command is 0 based, meaning 0 is the leftmost column."] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MoveToColumn (pub u16) ;
};
}
