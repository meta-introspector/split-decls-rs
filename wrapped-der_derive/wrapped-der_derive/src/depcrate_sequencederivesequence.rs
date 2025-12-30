// Generated macro for DeriveSequence (struct)
macro_rules! Depcrate_sequenceDeriveSequence {
() => {
// Module: crate::sequence
// Provides: {"DeriveSequence"}
// Dependencies: {}
# [doc = " Derive the `Sequence` trait for a struct"] pub (crate) struct DeriveSequence { # [doc = " Name of the sequence struct."] ident : Ident , # [doc = " Generics of the struct."] generics : Generics , # [doc = " Fields of the struct."] fields : Vec < SequenceField > , # [doc = " Error type for `DecodeValue` implementation."] error : ErrorType , }
};
}
