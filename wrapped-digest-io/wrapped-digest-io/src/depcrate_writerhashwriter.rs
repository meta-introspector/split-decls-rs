// Generated macro for HashWriter (struct)
macro_rules! Depcrate_writerHashWriter {
() => {
// Module: crate::writer
// Provides: {"HashWriter"}
// Dependencies: {}
# [doc = " Abstraction over a writer which hashes the data being written."] # [derive (Debug)] pub struct HashWriter < D : Digest , W : io :: Write > { writer : W , hasher : D , }
};
}
