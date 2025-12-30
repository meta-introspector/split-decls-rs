// Generated macro for Category (enum)
macro_rules! Depcrate_errorCategory {
() => {
// Module: crate::error
// Provides: {"Category"}
// Dependencies: {}
# [doc = " Categorizes the cause of a `serde_cbor::Error`."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum Category { # [doc = " The error was caused by a failure to read or write bytes on an IO stream."] Io , # [doc = " The error was caused by input that was not syntactically valid CBOR."] Syntax , # [doc = " The error was caused by input data that was semantically incorrect."] Data , # [doc = " The error was caused by prematurely reaching the end of the input data."] Eof , }
};
}
