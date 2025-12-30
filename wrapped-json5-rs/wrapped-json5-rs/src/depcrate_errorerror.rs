// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " A bare bones error type which currently just collapses all the underlying errors in to a single"] # [doc = " string... This is fine for displaying to the user, but not very useful otherwise. Work to be"] # [doc = " done here."] # [derive (Clone , Debug , PartialEq)] pub enum Error { # [doc = " Just shove everything in a single variant for now."] Message { # [doc = " The error message."] msg : String , # [doc = " The location of the error, if applicable."] location : Option < Location > , } , }
};
}
