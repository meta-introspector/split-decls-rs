// Generated macro for ConnectionError (struct)
macro_rules! DepcrateConnectionError {
() => {
// Module: crate
// Provides: {"ConnectionError"}
// Dependencies: {}
# [doc = " Represents information carried by `CONNECTION_CLOSE` frames."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct ConnectionError { # [doc = " Whether the error came from the application or the transport layer."] pub is_app : bool , # [doc = " The error code carried by the `CONNECTION_CLOSE` frame."] pub error_code : u64 , # [doc = " The reason carried by the `CONNECTION_CLOSE` frame."] pub reason : Vec < u8 > , }
};
}
