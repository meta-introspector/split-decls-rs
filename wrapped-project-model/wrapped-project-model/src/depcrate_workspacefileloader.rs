// Generated macro for FileLoader (type)
macro_rules! Depcrate_workspaceFileLoader {
() => {
// Module: crate::workspace
// Provides: {"FileLoader"}
// Dependencies: {}
pub type FileLoader < 'a > = & 'a mut dyn for < 'b > FnMut (& 'b AbsPath) -> Option < FileId > ;
};
}
