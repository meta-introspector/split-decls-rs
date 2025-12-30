// Generated macro for Chunk (struct)
macro_rules! Depcrate_connection_assemblerChunk {
() => {
// Module: crate::connection::assembler
// Provides: {"Chunk"}
// Dependencies: {}
# [doc = " A chunk of data from the receive stream"] # [derive (Debug , PartialEq , Eq)] pub struct Chunk { # [doc = " The offset in the stream"] pub offset : u64 , # [doc = " The contents of the chunk"] pub bytes : Bytes , }
};
}
