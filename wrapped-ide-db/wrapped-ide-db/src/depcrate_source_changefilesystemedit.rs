// Generated macro for FileSystemEdit (enum)
macro_rules! Depcrate_source_changeFileSystemEdit {
() => {
// Module: crate::source_change
// Provides: {"FileSystemEdit"}
// Dependencies: {}
# [derive (Debug , Clone)] pub enum FileSystemEdit { CreateFile { dst : AnchoredPathBuf , initial_contents : String } , MoveFile { src : FileId , dst : AnchoredPathBuf } , MoveDir { src : AnchoredPathBuf , src_id : FileId , dst : AnchoredPathBuf } , }
};
}
