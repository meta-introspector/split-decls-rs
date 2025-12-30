// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_serialErrorKind {
() => {
// Module: crate::serial
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " Serial error kind."] # [doc = ""] # [doc = " This represents a common set of serial operation errors. HAL implementations are"] # [doc = " free to define more specific or additional error types. However, by providing"] # [doc = " a mapping to these common serial errors, generic code can still react to them."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] # [non_exhaustive] pub enum ErrorKind { # [doc = " The peripheral receive buffer was overrun."] Overrun , # [doc = " Received data does not conform to the peripheral configuration."] # [doc = " Can be caused by a misconfigured device on either end of the serial line."] FrameFormat , # [doc = " Parity check failed."] Parity , # [doc = " Serial line is too noisy to read valid data."] Noise , # [doc = " A different error occurred. The original error may contain more information."] Other , }
};
}
