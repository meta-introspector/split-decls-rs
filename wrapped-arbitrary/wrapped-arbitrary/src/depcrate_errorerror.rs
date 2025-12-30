// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " An enumeration of buffer creation errors"] # [derive (Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub enum Error { # [doc = " No choices were provided to the Unstructured::choose call"] EmptyChoose , # [doc = " There was not enough underlying data to fulfill some request for raw"] # [doc = " bytes."] # [doc = ""] # [doc = " Note that outside of [`Unstructured::bytes`][crate::Unstructured::bytes],"] # [doc = " most APIs do *not* return this error when running out of underlying arbitrary bytes"] # [doc = " but silently return some default value instead."] NotEnoughData , # [doc = " The input bytes were not of the right format"] IncorrectFormat , }
};
}
