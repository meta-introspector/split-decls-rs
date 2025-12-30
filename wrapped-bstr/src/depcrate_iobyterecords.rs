// Generated macro for ByteRecords (struct)
macro_rules! Depcrate_ioByteRecords {
() => {
// Module: crate::io
// Provides: {"ByteRecords"}
// Dependencies: {}
# [doc = " An iterator over records from an instance of"] # [doc = " [`std::io::BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)."] # [doc = ""] # [doc = " A byte record is any sequence of bytes terminated by a particular byte"] # [doc = " chosen by the caller. For example, NUL separated byte strings are said to"] # [doc = " be NUL-terminated byte records."] # [doc = ""] # [doc = " This iterator is generally created by calling the"] # [doc = " [`byte_records`](trait.BufReadExt.html#method.byte_records)"] # [doc = " method on the"] # [doc = " [`BufReadExt`](trait.BufReadExt.html)"] # [doc = " trait."] # [derive (Debug)] pub struct ByteRecords < B > { buf : B , terminator : u8 , }
};
}
