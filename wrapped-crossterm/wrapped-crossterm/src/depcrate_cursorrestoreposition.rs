// Generated macro for RestorePosition (struct)
macro_rules! Depcrate_cursorRestorePosition {
() => {
// Module: crate::cursor
// Provides: {"RestorePosition"}
// Dependencies: {}
# [doc = " A command that restores the saved terminal cursor position."] # [doc = ""] # [doc = " See the [SavePosition](./struct.SavePosition.html) command."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " - The cursor position is stored globally."] # [doc = " - Commands must be executed/queued for execution otherwise they do nothing."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct RestorePosition ;
};
}
