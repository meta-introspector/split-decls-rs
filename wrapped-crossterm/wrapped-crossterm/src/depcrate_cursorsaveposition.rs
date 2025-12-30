// Generated macro for SavePosition (struct)
macro_rules! Depcrate_cursorSavePosition {
() => {
// Module: crate::cursor
// Provides: {"SavePosition"}
// Dependencies: {}
# [doc = " A command that saves the current terminal cursor position."] # [doc = ""] # [doc = " See the [RestorePosition](./struct.RestorePosition.html) command."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " - The cursor position is stored globally."] # [doc = " - Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct SavePosition ;
};
}
