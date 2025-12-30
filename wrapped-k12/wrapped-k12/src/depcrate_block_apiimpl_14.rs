// Generated macro for impl_14 (impl)
macro_rules! Depcrate_block_apiimpl_14 {
() => {
// Module: crate::block_api
// Provides: {"impl_14"}
// Dependencies: {}
impl UpdateCore for KangarooTwelveCore < '_ > { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { for block in blocks { if self . bufpos == CHUNK_SIZE { self . process_chunk () ; } self . buffer [self . bufpos .. self . bufpos + 128] . clone_from_slice (block) ; self . bufpos += 128 ; } } }
};
}
