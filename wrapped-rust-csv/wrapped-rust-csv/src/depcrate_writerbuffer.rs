// Generated macro for Buffer (struct)
macro_rules! Depcrate_writerBuffer {
() => {
// Module: crate::writer
// Provides: {"Buffer"}
// Dependencies: {}
# [doc = " A simple internal buffer for buffering writes."] # [doc = ""] # [doc = " We need this because the `csv_core` APIs want to write into a `&mut [u8]`,"] # [doc = " which is not available with the `std::io::BufWriter` API."] # [derive (Debug)] struct Buffer { # [doc = " The contents of the buffer."] buf : Vec < u8 > , # [doc = " The number of bytes written to the buffer."] len : usize , }
};
}
