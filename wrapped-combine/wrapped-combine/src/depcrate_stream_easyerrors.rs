// Generated macro for Errors (struct)
macro_rules! Depcrate_stream_easyErrors {
() => {
// Module: crate::stream::easy
// Provides: {"Errors"}
// Dependencies: {}
# [doc = " Struct which hold information about an error that occurred at a specific position."] # [doc = " Can hold multiple instances of `Error` if more that one error occurred in the same position."] # [derive (Debug , PartialEq)] pub struct Errors < T , R , P > { # [doc = " The position where the error occurred"] pub position : P , # [doc = " A vector containing specific information on what errors occurred at `position`. Usually"] # [doc = " a fully formed message contains one `Unexpected` error and one or more `Expected` errors."] # [doc = " `Message` and `Other` may also appear (`combine` never generates these errors on its own)"] # [doc = " and may warrant custom handling."] pub errors : Vec < Error < T , R > > , }
};
}
