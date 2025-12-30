// Generated macro for WrappedLine (struct)
macro_rules! Depcrate_reflowWrappedLine {
() => {
// Module: crate::reflow
// Provides: {"WrappedLine"}
// Dependencies: {}
# [doc = " A line that has been wrapped to a certain width."] pub struct WrappedLine < 'lend , 'text > { # [doc = " One line reflowed to the correct width"] pub graphemes : & 'lend [StyledGrapheme < 'text >] , # [doc = " The width of the line"] pub width : u16 , # [doc = " Whether the line was aligned left or right"] pub alignment : Alignment , }
};
}
