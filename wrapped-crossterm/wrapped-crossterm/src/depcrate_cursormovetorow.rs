// Generated macro for MoveToRow (struct)
macro_rules! Depcrate_cursorMoveToRow {
() => {
// Module: crate::cursor
// Provides: {"MoveToRow"}
// Dependencies: {}
# [doc = " A command that moves the terminal cursor to the given row on the current column."] # [doc = ""] # [doc = " # Notes"] # [doc = " * This command is 0 based, meaning 0 is the topmost row."] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MoveToRow (pub u16) ;
};
}
