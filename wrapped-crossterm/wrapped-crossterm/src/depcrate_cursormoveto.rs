// Generated macro for MoveTo (struct)
macro_rules! Depcrate_cursorMoveTo {
() => {
// Module: crate::cursor
// Provides: {"MoveTo"}
// Dependencies: {}
# [doc = " A command that moves the terminal cursor to the given position (column, row)."] # [doc = ""] # [doc = " # Notes"] # [doc = " * Top left cell is represented as `0,0`."] # [doc = " * Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct MoveTo (pub u16 , pub u16) ;
};
}
