// Generated macro for Error (enum)
macro_rules! Depcrate_proto_errorError {
() => {
// Module: crate::proto::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Either an H2 reason  or an I/O error"] # [derive (Clone , Debug)] pub enum Error { Reset (StreamId , Reason , Initiator) , GoAway (Bytes , Reason , Initiator) , Io (io :: ErrorKind , Option < String >) , }
};
}
