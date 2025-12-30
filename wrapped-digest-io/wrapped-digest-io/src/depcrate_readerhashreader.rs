// Generated macro for HashReader (struct)
macro_rules! Depcrate_readerHashReader {
() => {
// Module: crate::reader
// Provides: {"HashReader"}
// Dependencies: {}
# [doc = " Abstraction over a reader which hashes the data being read"] # [derive (Debug)] pub struct HashReader < D : Digest , R : io :: Read > { reader : R , hasher : D , }
};
}
