// Generated macro for File (struct)
macro_rules! Depcrate_fileFile {
() => {
// Module: crate::file
// Provides: {"File"}
// Dependencies: {}
# [doc = " A configuration source backed up by a file."] # [doc = ""] # [doc = " It supports optional automatic file format discovery."] # [derive (Clone , Debug)] # [must_use] pub struct File < T , F > { source : T , # [doc = " Format of file (which dictates what driver to use)."] format : Option < F > , # [doc = " A required File will error if it cannot be found"] required : bool , }
};
}
