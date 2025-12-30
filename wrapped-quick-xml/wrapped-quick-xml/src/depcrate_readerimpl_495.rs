// Generated macro for impl_495 (impl)
macro_rules! Depcrate_readerimpl_495 {
() => {
// Module: crate::reader
// Provides: {"impl_495"}
// Dependencies: {}
impl < 'r , R > BinaryStream < 'r , R > { # [doc = " Returns current position in bytes in the original source."] # [inline] pub const fn offset (& self) -> u64 { * self . offset } # [doc = " Gets a reference to the underlying reader."] # [inline] pub const fn get_ref (& self) -> & R { self . inner } # [doc = " Gets a mutable reference to the underlying reader."] # [doc = ""] # [doc = " Avoid read from this reader because this will not update reader's position"] # [doc = " and will lead to incorrect positions of errors. Read from this stream instead."] # [inline] pub fn get_mut (& mut self) -> & mut R { self . inner } }
};
}
