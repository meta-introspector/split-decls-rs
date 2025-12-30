// Generated macro for ClearType (enum)
macro_rules! Depcrate_backendClearType {
() => {
// Module: crate::backend
// Provides: {"ClearType"}
// Dependencies: {}
# [doc = " Enum representing the different types of clearing operations that can be performed"] # [doc = " on the terminal screen."] # [derive (Debug , Display , EnumString , Clone , Copy , Eq , PartialEq , Hash)] pub enum ClearType { # [doc = " Clear the entire screen."] All , # [doc = " Clear everything after the cursor."] AfterCursor , # [doc = " Clear everything before the cursor."] BeforeCursor , # [doc = " Clear the current line."] CurrentLine , # [doc = " Clear everything from the cursor until the next newline."] UntilNewLine , }
};
}
