// Generated macro for impl_304 (impl)
macro_rules! Depcrateimpl_304 {
() => {
// Module: crate
// Provides: {"impl_304"}
// Dependencies: {}
impl ChunkFooter { fn as_raw_parts (& self) -> (* const u8 , usize) { let data = self . data . as_ptr () as * const u8 ; let ptr = self . ptr . get () . as_ptr () as * const u8 ; debug_assert ! (data <= ptr) ; debug_assert ! (ptr <= self as * const ChunkFooter as * const u8) ; let len = unsafe { (self as * const ChunkFooter as * const u8) . offset_from (ptr) as usize } ; (ptr , len) } # [doc = " Is this chunk the last empty chunk?"] fn is_empty (& self) -> bool { ptr :: eq (self , EMPTY_CHUNK . get () . as_ptr ()) } }
};
}
