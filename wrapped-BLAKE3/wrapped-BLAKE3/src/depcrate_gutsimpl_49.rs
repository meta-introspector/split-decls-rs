// Generated macro for impl_49 (impl)
macro_rules! Depcrate_gutsimpl_49 {
() => {
// Module: crate::guts
// Provides: {"impl_49"}
// Dependencies: {}
impl ChunkState { pub fn new (chunk_counter : u64) -> Self { Self (crate :: ChunkState :: new (crate :: IV , chunk_counter , 0 , crate :: platform :: Platform :: detect () ,)) } # [inline] pub fn len (& self) -> usize { self . 0 . count () } # [inline] pub fn update (& mut self , input : & [u8]) -> & mut Self { self . 0 . update (input) ; self } pub fn finalize (& self , is_root : bool) -> crate :: Hash { let output = self . 0 . output () ; if is_root { output . root_hash () } else { output . chaining_value () . into () } } }
};
}
