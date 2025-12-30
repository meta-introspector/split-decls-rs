// Generated macro for ByteSink (trait)
macro_rules! Depcrate_isa_x64_encodingByteSink {
() => {
// Module: crate::isa::x64::encoding
// Provides: {"ByteSink"}
// Dependencies: {}
# [doc = " The encoding formats in this module all require a way of placing bytes into"] # [doc = " a buffer."] pub trait ByteSink { # [doc = " Add 1 byte to the code section."] fn put1 (& mut self , _ : u8) ; # [doc = " Add 2 bytes to the code section."] fn put2 (& mut self , _ : u16) ; # [doc = " Add 4 bytes to the code section."] fn put4 (& mut self , _ : u32) ; # [doc = " Add 8 bytes to the code section."] fn put8 (& mut self , _ : u64) ; }
};
}
