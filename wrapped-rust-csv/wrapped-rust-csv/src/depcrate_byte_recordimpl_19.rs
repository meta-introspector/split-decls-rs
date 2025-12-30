// Generated macro for impl_19 (impl)
macro_rules! Depcrate_byte_recordimpl_19 {
() => {
// Module: crate::byte_record
// Provides: {"impl_19"}
// Dependencies: {}
impl Position { # [doc = " Returns a new position initialized to the start value."] # [inline] pub fn new () -> Position { Position { byte : 0 , line : 1 , record : 0 } } # [doc = " The byte offset, starting at `0`, of this position."] # [inline] pub fn byte (& self) -> u64 { self . byte } # [doc = " The line number, starting at `1`, of this position."] # [inline] pub fn line (& self) -> u64 { self . line } # [doc = " The record index, starting with the first record at `0`."] # [inline] pub fn record (& self) -> u64 { self . record } # [doc = " Set the byte offset of this position."] # [inline] pub fn set_byte (& mut self , byte : u64) -> & mut Position { self . byte = byte ; self } # [doc = " Set the line number of this position."] # [doc = ""] # [doc = " If the line number is less than `1`, then this method panics."] # [inline] pub fn set_line (& mut self , line : u64) -> & mut Position { assert ! (line > 0) ; self . line = line ; self } # [doc = " Set the record index of this position."] # [inline] pub fn set_record (& mut self , record : u64) -> & mut Position { self . record = record ; self } }
};
}
