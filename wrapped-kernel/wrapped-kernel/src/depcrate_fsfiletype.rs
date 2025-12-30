// Generated macro for FileType (enum)
macro_rules! Depcrate_fsFileType {
() => {
// Module: crate::fs
// Provides: {"FileType"}
// Dependencies: {}
# [derive (TryFromPrimitive , IntoPrimitive , PartialEq , Eq , Clone , Copy , Debug)] # [repr (u8)] pub enum FileType { Unknown = 0 , Fifo = 1 , CharacterDevice = 2 , Directory = 4 , BlockDevice = 6 , RegularFile = 8 , SymbolicLink = 10 , Socket = 12 , Whiteout = 14 , }
};
}
