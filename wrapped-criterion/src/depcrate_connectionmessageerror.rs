// Generated macro for MessageError (enum)
macro_rules! Depcrate_connectionMessageError {
() => {
// Module: crate::connection
// Provides: {"MessageError"}
// Dependencies: {}
# [derive (Debug)] pub enum MessageError { Deserialization (ciborium :: de :: Error < std :: io :: Error >) , Serialization (ciborium :: ser :: Error < std :: io :: Error >) , Io (std :: io :: Error) , }
};
}
