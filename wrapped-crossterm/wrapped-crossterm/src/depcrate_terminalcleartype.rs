// Generated macro for ClearType (enum)
macro_rules! Depcrate_terminalClearType {
() => {
// Module: crate::terminal
// Provides: {"ClearType"}
// Dependencies: {}
# [doc = " Different ways to clear the terminal buffer."] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] # [derive (Copy , Clone , Debug , PartialEq , Eq , Ord , PartialOrd , Hash)] pub enum ClearType { # [doc = " All cells."] All , # [doc = " All plus history"] Purge , # [doc = " All cells from the cursor position downwards."] FromCursorDown , # [doc = " All cells from the cursor position upwards."] FromCursorUp , # [doc = " All cells at the cursor row."] CurrentLine , # [doc = " All cells from the cursor position until the new line."] UntilNewLine , }
};
}
