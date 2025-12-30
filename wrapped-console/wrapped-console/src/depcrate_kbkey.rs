// Generated macro for Key (enum)
macro_rules! Depcrate_kbKey {
() => {
// Module: crate::kb
// Provides: {"Key"}
// Dependencies: {}
# [doc = " Key mapping"] # [doc = ""] # [doc = " This is an incomplete mapping of keys that are supported for reading"] # [doc = " from the keyboard."] # [non_exhaustive] # [derive (Clone , PartialEq , Eq , Debug , Hash)] pub enum Key { Unknown , # [doc = " Unrecognized sequence containing Esc and a list of chars"] UnknownEscSeq (Vec < char >) , ArrowLeft , ArrowRight , ArrowUp , ArrowDown , Enter , Escape , Backspace , Home , End , Tab , BackTab , Alt , Del , Shift , Insert , PageUp , PageDown , Char (char) , CtrlC , }
};
}
