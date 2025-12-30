// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [derive (Debug)] pub enum Error { # [doc = " Error from the Corn parser"] Corn (CornError) , # [doc = " Error while reading the input file from disk"] ReadingFile (io :: Error) , # [doc = " Error when serializing output"] Serializing (String) , }
};
}
