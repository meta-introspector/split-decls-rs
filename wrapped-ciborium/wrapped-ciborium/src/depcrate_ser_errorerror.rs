// Generated macro for Error (enum)
macro_rules! Depcrate_ser_errorError {
() => {
// Module: crate::ser::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An error occurred during serialization"] # [derive (Clone , Debug)] pub enum Error < T > { # [doc = " An error occurred while writing bytes"] # [doc = ""] # [doc = " Contains the underlying error returned while writing."] Io (T) , # [doc = " An error indicating a value that cannot be serialized"] # [doc = ""] # [doc = " Contains a description of the problem."] Value (String) , }
};
}
