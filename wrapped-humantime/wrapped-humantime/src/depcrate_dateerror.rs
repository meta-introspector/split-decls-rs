// Generated macro for Error (enum)
macro_rules! Depcrate_dateError {
() => {
// Module: crate::date
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Error parsing datetime (timestamp)"] # [derive (Debug , PartialEq , Clone , Copy)] pub enum Error { # [doc = " Numeric component is out of range"] OutOfRange , # [doc = " Bad character where digit is expected"] InvalidDigit , # [doc = " Other formatting errors"] InvalidFormat , }
};
}
