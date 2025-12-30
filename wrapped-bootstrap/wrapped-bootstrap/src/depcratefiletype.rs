// Generated macro for FileType (enum)
macro_rules! DepcrateFileType {
() => {
// Module: crate
// Provides: {"FileType"}
// Dependencies: {}
# [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum FileType { # [doc = " An executable binary file (like a `.exe`)."] Executable , # [doc = " A native, binary library file (like a `.so`, `.dll`, `.a`, `.lib` or `.o`)."] NativeLibrary , # [doc = " An executable (non-binary) script file (like a `.py` or `.sh`)."] Script , # [doc = " Any other regular file that is non-executable."] Regular , }
};
}
