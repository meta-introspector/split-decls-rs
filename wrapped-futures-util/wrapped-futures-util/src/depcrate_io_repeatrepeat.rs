// Generated macro for repeat (function)
macro_rules! Depcrate_io_repeatrepeat {
() => {
// Module: crate::io::repeat
// Provides: {"repeat"}
// Dependencies: {}
# [doc = " Creates an instance of a reader that infinitely repeats one byte."] # [doc = ""] # [doc = " All reads from this reader will succeed by filling the specified buffer with"] # [doc = " the given byte."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::io::{self, AsyncReadExt};"] # [doc = ""] # [doc = " let mut buffer = [0; 3];"] # [doc = " let mut reader = io::repeat(0b101);"] # [doc = " reader.read_exact(&mut buffer).await.unwrap();"] # [doc = " assert_eq!(buffer, [0b101, 0b101, 0b101]);"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();"] # [doc = " ```"] pub fn repeat (byte : u8) -> Repeat { Repeat { byte } }
};
}
