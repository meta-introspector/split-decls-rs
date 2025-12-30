// Generated macro for GraphFromFileError (enum)
macro_rules! Depcrate_astGraphFromFileError {
() => {
// Module: crate::ast
// Provides: {"GraphFromFileError"}
// Dependencies: {}
# [doc = " An error that can occur when reading from a file."] # [derive (Debug)] pub enum GraphFromFileError < 'a > { # [doc = " The error occured when manipulating the file (e.g. the file does not exist)."] FileError (IOError) , # [doc = " The error occured when parsing the file (e.g. the pest parser returned an error)."] PestParseError (PestError) , # [doc = " The error occured when traversing the `Rule`s tree returned by the pest parser."] # [doc = " Such error occuring is likely a bug or a missing feature of the library."] ParseError (ParseError < 'a >) , }
};
}
